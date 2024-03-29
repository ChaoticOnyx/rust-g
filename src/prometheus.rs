use prometheus_client::{
    encoding::{EncodeCounterValue, EncodeGaugeValue},
    metrics::{counter::Counter, family::Family, gauge::Gauge},
    registry::Registry,
};
use std::{
    any::Any,
    cell::RefCell,
    collections::BTreeMap,
    fmt::Debug,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
        Mutex, MutexGuard,
    },
};
use tiny_http::{Header, Response, Server};

static REGISTRY: Mutex<RefCell<Option<Registry>>> = Mutex::new(RefCell::new(None));
static METRICS: Mutex<RefCell<BTreeMap<String, Box<dyn Any + Send>>>> =
    Mutex::new(RefCell::new(BTreeMap::new()));
static ENDPOINT_RUNS: AtomicBool = AtomicBool::new(false);
static LABELS: Mutex<RefCell<BTreeMap<String, String>>> = Mutex::new(RefCell::new(BTreeMap::new()));

fn spawn_endpoint(port: &str) {
    if ENDPOINT_RUNS.swap(true, Ordering::Relaxed) {
        return;
    }

    let addr = format!("0.0.0.0:{port}");

    std::thread::spawn(move || {
        let server = Server::http(addr).unwrap();

        for req in server.incoming_requests() {
            if !req.url().starts_with("/metrics") {
                continue;
            }

            let registry = get_registry();
            let registry = registry.borrow();
            let registry = registry.as_ref().unwrap();

            let mut buf = String::new();

            if prometheus_client::encoding::text::encode(&mut buf, registry).is_err() {
                continue;
            }

            let response = Response::from_string(buf).with_header(
                Header::from_str(
                    "Content-Type: application/openmetrics-text; version=1.0.0; charset=utf-8",
                )
                .unwrap(),
            );

            req.respond(response).unwrap();
        }

        ENDPOINT_RUNS.store(false, Ordering::Relaxed);
    });
}

fn get_registry() -> MutexGuard<'static, RefCell<Option<Registry>>> {
    let registry = REGISTRY.lock().unwrap();

    {
        let mut registry = registry.borrow_mut();

        if registry.is_none() {
            *registry = Some(Registry::with_prefix("byond"));
        }
    }

    registry
}

fn do_with_metric<M>(id: &str, label_set: &str, callback: impl Fn(&M))
where
    M: 'static,
{
    let metrics = METRICS.lock().unwrap();
    let metrics = metrics.borrow();

    let Some(family) = metrics
        .get(id)
        .and_then(|v| v.downcast_ref::<Family<Vec<(String, String)>, M>>())
    else {
        return;
    };

    let label_map = match label_set {
        "null" | "[]" => BTreeMap::new(),
        _ => serde_json::from_str::<BTreeMap<&str, &str>>(label_set).unwrap(),
    };

    let mut label_set = Vec::new();

    for (key, value) in label_map {
        label_set.push((key.to_string(), value.to_string()));
    }

    let global_labels = LABELS.lock().unwrap();
    let global_labels = global_labels.borrow();

    for (key, value) in &*global_labels {
        label_set.push((key.to_string(), value.to_string()));
    }

    let metric = family.get_or_create(&label_set);

    callback(&metric)
}

fn counter_register<N, A>(id: &str, desc: &str)
where
    N: Send + Sync + Debug + EncodeCounterValue + 'static,
    A: Default + Send + Sync + Debug + prometheus_client::metrics::counter::Atomic<N> + 'static,
{
    let metrics = METRICS.lock().unwrap();
    let mut metrics = metrics.borrow_mut();

    if metrics.contains_key(id) {
        return;
    }

    let registry = get_registry();
    let mut registry = registry.borrow_mut();
    let registry = registry.as_mut().unwrap();

    let family = Family::<Vec<(String, String)>, Counter<N, A>>::default();

    metrics.insert(id.to_string(), Box::new(family.clone()));
    registry.register(id, desc, family);
}

fn counter_inc<N, A>(id: &str, label_set: &str)
where
    N: 'static,
    A: prometheus_client::metrics::counter::Atomic<N> + 'static,
{
    do_with_metric::<Counter<N, A>>(id, label_set, |counter| {
        counter.inc();
    });
}

fn counter_inc_by<N, A>(id: &str, value: &str, label_set: &str)
where
    N: Copy + FromStr + 'static,
    A: prometheus_client::metrics::counter::Atomic<N> + 'static,
{
    let Ok(value) = value.parse::<N>() else {
        return;
    };

    do_with_metric::<Counter<N, A>>(id, label_set, |counter| {
        counter.inc_by(value);
    });
}

fn gauge_register<N, A>(id: &str, desc: &str)
where
    N: Send + Sync + Debug + EncodeGaugeValue + 'static,
    A: Default + Send + Sync + Debug + prometheus_client::metrics::gauge::Atomic<N> + 'static,
{
    let metrics = METRICS.lock().unwrap();
    let mut metrics = metrics.borrow_mut();

    if metrics.contains_key(id) {
        return;
    }

    let registry = get_registry();
    let mut registry = registry.borrow_mut();
    let registry = registry.as_mut().unwrap();

    let gauge = Family::<Vec<(String, String)>, Gauge<N, A>>::default();

    metrics.insert(id.to_string(), Box::new(gauge.clone()));
    registry.register(id, desc, gauge);
}

fn gauge_inc<N, A>(id: &str, label_set: &str)
where
    N: 'static,
    A: prometheus_client::metrics::gauge::Atomic<N> + 'static,
{
    do_with_metric::<Gauge<N, A>>(id, label_set, |counter| {
        counter.inc();
    });
}

fn gauge_inc_by<N, A>(id: &str, value: &str, label_set: &str)
where
    N: Copy + FromStr + 'static,
    A: prometheus_client::metrics::gauge::Atomic<N> + 'static,
{
    let Ok(value) = value.parse::<N>() else {
        return;
    };

    do_with_metric::<Gauge<N, A>>(id, label_set, |counter| {
        counter.inc_by(value);
    });
}

fn gauge_dec<N, A>(id: &str, label_set: &str)
where
    N: 'static,
    A: prometheus_client::metrics::gauge::Atomic<N> + 'static,
{
    do_with_metric::<Gauge<N, A>>(id, label_set, |counter| {
        counter.dec();
    });
}

fn gauge_dec_by<N, A>(id: &str, value: &str, label_set: &str)
where
    N: Copy + FromStr + 'static,
    A: prometheus_client::metrics::gauge::Atomic<N> + 'static,
{
    let Ok(value) = value.parse::<N>() else {
        return;
    };

    do_with_metric::<Gauge<N, A>>(id, label_set, |counter| {
        counter.dec_by(value);
    });
}

fn gauge_set<N, A>(id: &str, value: &str, label_set: &str)
where
    N: Copy + FromStr + 'static,
    A: prometheus_client::metrics::gauge::Atomic<N> + 'static,
{
    let Ok(value) = value.parse::<N>() else {
        return;
    };

    do_with_metric::<Gauge<N, A>>(id, label_set, |counter| {
        counter.set(value);
    });
}

fn set_labels(label_set: &str) {
    let Ok(label_set) = serde_json::from_str::<BTreeMap<String, String>>(label_set) else {
        return;
    };

    let labels = LABELS.lock().unwrap();
    let mut labels = labels.borrow_mut();

    let _ = std::mem::replace(&mut *labels, label_set);
}

byond_fn!(
    fn prom_init(port) {
        {
            let registry = REGISTRY.lock().unwrap();
            registry.borrow_mut().take();
        }

        {
            let metrics = METRICS.lock().unwrap();
            metrics.borrow_mut().clear();
        }

        spawn_endpoint(port);

        Some("")
    }
);

byond_fn!(fn prom_set_labels(labels) {
    set_labels(labels);

    Some("")
});

// Counters

byond_fn!(fn prom_counter_register(id, desc) {
    counter_register::<u64, AtomicU64>(id, desc);

    Some("")
});

byond_fn!(fn prom_counter_inc(id, label_set) {
    counter_inc::<u64, AtomicU64>(id, label_set);

    Some("")
});

byond_fn!(fn prom_counter_inc_by(id, value, label_set) {
    counter_inc_by::<u64, AtomicU64>(id, value, label_set);

    Some("")
});

// Int gauges

byond_fn!(fn prom_gauge_int_register(id, desc) {
    gauge_register::<i64, AtomicI64>(id, desc);

    Some("")
});

byond_fn!(fn prom_gauge_int_inc(id, label_set) {
    gauge_inc::<i64, AtomicI64>(id, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_int_inc_by(id, value, label_set) {
    gauge_inc_by::<i64, AtomicI64>(id, value, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_int_dec(id, label_set) {
    gauge_dec::<i64, AtomicI64>(id, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_int_dec_by(id, value, label_set) {
    gauge_dec_by::<i64, AtomicI64>(id, value, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_int_set(id, value, label_set) {
    gauge_set::<i64, AtomicI64>(id, value, label_set);

    Some("")
});

// Float gauges

byond_fn!(fn prom_gauge_float_register(id, desc) {
    gauge_register::<f64, AtomicU64>(id, desc);

    Some("")
});

byond_fn!(fn prom_gauge_float_inc(id, label_set) {
    gauge_inc::<f64, AtomicU64>(id, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_float_inc_by(id, value, label_set) {
    gauge_inc_by::<f64, AtomicU64>(id, value, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_float_dec(id, label_set) {
    gauge_dec::<f64, AtomicU64>(id, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_float_dec_by(id, value, label_set) {
    gauge_dec_by::<f64, AtomicU64>(id, value, label_set);

    Some("")
});

byond_fn!(fn prom_gauge_float_set(id, value, label_set) {
    gauge_set::<f64, AtomicU64>(id, value, label_set);

    Some("")
});
