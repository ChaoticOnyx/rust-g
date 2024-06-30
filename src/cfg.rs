use config::{builder::DefaultState, Config, ConfigBuilder, Environment, File};
use glob::glob;
use std::sync::RwLock;

static CONFIG: RwLock<Option<Config>> = RwLock::new(None);
static BUILDER: RwLock<Option<ConfigBuilder<DefaultState>>> = RwLock::new(None);
const REGISTERED_FORMATS: &[&str] = &["toml", "yaml", "json", "json5", "ini"];

fn begin_builder() {
    let mut builder = BUILDER.try_write().unwrap();

    *builder = Some(Config::builder());
}

fn add_source_glob(pattern: &str) -> Result<(), String> {
    let mut builder = BUILDER.try_write().unwrap();

    let files: Vec<File<_, _>> = glob(pattern)
        .map_err(|err| err.to_string())?
        .filter_map(|path| {
            let Ok(path) = path else {
                return None;
            };

            let Some(ext) = path.extension() else {
                return None;
            };

            let ext = ext.to_string_lossy().to_string();

            if REGISTERED_FORMATS.contains(&ext.as_str()) {
                Some(path)
            } else {
                None
            }
        })
        .map(|path| File::from(path))
        .collect();

    *builder = builder
        .take()
        .and_then(|builder| Some(builder.add_source(files)));

    Ok(())
}

fn add_source_file(name: &str) -> Result<(), String> {
    let mut builder = BUILDER.try_write().unwrap();

    *builder = builder
        .take()
        .and_then(|builder| Some(builder.add_source(File::with_name(name))));

    Ok(())
}

fn add_source_env(prefix: &str, separator: &str) -> Result<(), String> {
    let mut builder = BUILDER.try_write().unwrap();

    if !prefix.is_empty() {
        *builder = builder.take().and_then(|builder| {
            Some(builder.add_source(Environment::with_prefix(prefix).separator(separator)))
        });
    }

    Ok(())
}

fn end_builder() -> Result<(), String> {
    let mut builder = BUILDER.try_write().unwrap();
    let mut config = CONFIG.try_write().unwrap();

    let builder = builder
        .take()
        .ok_or_else(|| String::from("Call 'cfg_begin_builder' first"))?;

    *config = Some(builder.build().map_err(|err| err.to_string())?);

    Ok(())
}

fn try_deserialize() -> Result<String, String> {
    let mut config = CONFIG.try_write().unwrap();
    let config = config
        .take()
        .ok_or_else(|| String::from("Call 'cfg_end_builder' first"))?;
    let config: serde_json::Value = config.try_deserialize().map_err(|err| err.to_string())?;

    serde_json::to_string(&config).map_err(|err| err.to_string())
}

byond_fn!(
    fn cfg_begin_builder() {
        begin_builder();

        Some("")
    }
);

byond_fn!(fn cfg_add_source_glob(pattern) {
    match add_source_glob(pattern) {
        Ok(_) => Some(String::new()),
        Err(err) => Some(err)
    }
});

byond_fn!(fn cfg_add_source_file(name) {
    match add_source_file(name) {
        Ok(_) => Some(String::new()),
        Err(err) => Some(err)
    }
});

byond_fn!(fn cfg_add_source_env(prefix, separator) {
    match add_source_env(prefix, separator) {
        Ok(_) => Some(String::new()),
        Err(err) => Some(err)
    }
});

byond_fn!(
    fn cfg_end_builder() {
        match end_builder() {
            Ok(_) => Some(String::new()),
            Err(err) => Some(err),
        }
    }
);

byond_fn!(
    fn cfg_try_deserialize() {
        match try_deserialize() {
            Ok(json) => Some(json),
            Err(err) => Some(err),
        }
    }
);
