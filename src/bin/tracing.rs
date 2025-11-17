use std::fmt::Debug;
use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::{fs::File, thread::sleep, time::Duration};
use tracing::field::{Field, Visit};
use tracing::{instrument, trace, Event, Span, Subscriber};
use tracing_subscriber::layer::{Context, SubscriberExt};
use tracing_subscriber::{fmt, fmt::writer::BoxMakeWriter, EnvFilter, Layer};

mod a {
    use crate::work;
    use tracing::{info, instrument, trace};

    #[instrument] // a span is created and entered for the whole function; args are recorded
    pub(crate) fn outer(n: i32) {
        trace!("about to call `work`"); // lightweight event inside the current span
        let res = work(n, 15);
        info!(result = res, "done");
    }
}

#[instrument(skip(delay_ms), fields(test = 0))] // record x, but don't record delay_ms
fn work(x: i32, delay_ms: u64) -> i32 {
    trace!("starting inner work");
    sleep(Duration::from_millis(delay_ms));
    let y = x * 2;
    Span::current().record("test", &42i64);
    trace!(y, "after compute"); // ? uses Debug formatting for the field
    y
}

fn main() {
    let file = File::create("trace_output.json").expect("Unable to create trace file");

    // print to stdout
    let console_layer = fmt::layer()
        .with_writer(stdout)
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .with_filter(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()));

    // print log to json file
    let file_layer = fmt::Layer::new()
        .with_writer(BoxMakeWriter::new(file))
        .json()
        .with_span_list(false)
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::filter_fn(|meta| {
            meta.target().starts_with("tracing::a")
        }));

    let subscriber = tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .with(CsvValueLayer::new());

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    a::outer(21);
}

struct CsvValueLayer {
    writer: Arc<Mutex<csv::Writer<File>>>,
}

impl CsvValueLayer {
    fn new() -> Self {
        let file = File::create("trace_output.csv").expect("Unable to create trace file");
        let mut writer = csv::Writer::from_writer(file);
        writer.write_record(["y", "result"]).unwrap();

        Self {
            writer: Arc::new(Mutex::new(writer)),
        }
    }
}

impl<S> Layer<S> for CsvValueLayer
where
    S: Subscriber,
{
    fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>) {
        let current = _ctx.current_span();
        // println!("{:?}", current);
        // current.metadata().as_mut().unwrap().fields().

        let mut visit = FieldVisitor::default();
        _event.record(&mut visit);
        self.writer
            .lock()
            .unwrap()
            .write_record([visit.y.to_string(), visit.result.to_string()])
            .unwrap();

        self.writer.lock().unwrap().flush().unwrap();
    }
}

#[derive(Default)]
struct FieldVisitor {
    y: i64,
    result: i64,
}

impl Visit for FieldVisitor {
    fn record_i64(&mut self, field: &Field, value: i64) {
        let name = field.name();
        // println!("name: {}", name);
        // println!("value: {:?}", value);
        match name {
            "y" => self.y = format!("{:?}", value).parse::<i64>().unwrap(),
            "result" => self.result = format!("{:?}", value).parse::<i64>().unwrap(),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        // println!("DEBUG: Field {:?} has value {:?}", field, value);
        // let name = field.name();
        // println!("name: {}", name);
        // println!("value: {:?}", value);
        // match name {
        //     "y" => self.y = format!("{:?}", value).parse::<i64>().unwrap(),
        //     "result" => self.result = format!("{:?}", value).parse::<i64>().unwrap(),
        //     _ => {}
        // }
    }
}

struct CSVLayer {}

impl<S> Layer<S> for CSVLayer where S: Subscriber {}
