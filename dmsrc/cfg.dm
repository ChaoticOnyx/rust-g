/// Call this first to initialize a config builder.
/// Then use the `rustg_cfg_add_*` macros to configure the builder
/// (calling these macros before `begin_builder` of `end_builder` will cause an error).
/// When you are done with configuring the builder - call `rustg_cfg_end_builder`,
/// then you can call `rustg_cfg_try_deserialize` which will return a JSON string with your config.
///
/// **You can only call `rustg_cfg_try_deserialize` only once,
/// then you have to do everything from `rustg_cfg_begin_builder` and to the end again before you can call this again.**
#define rustg_cfg_begin_builder(...) RUSTG_CALL(RUST_G, "cfg_begin_builder")()

/// Add files that match the glob pattern, for example: `config/*`
#define rustg_cfg_add_source_glob(pattern) RUSTG_CALL(RUST_G, "cfg_add_source_glob")(pattern)

/// Add files that match the glob pattern, for example: `config/*`Add the file with the specified path. The extension is optional.
#define rustg_cfg_add_source_file(name) RUSTG_CALL(RUST_G, "cfg_add_source_file")(name)

/// Use environment variables. The prefix and the separator are optional.
#define rustg_cfg_add_source_env(prefix, separator) RUSTG_CALL(RUST_G, "cfg_add_source_env")(isnull(prefix) ? "" : prefix, isnull(separator) ? "" : separator)

#define rustg_cfg_end_builder(...) RUSTG_CALL(RUST_G, "cfg_end_builder")()

/// Returns a JSON string.
#define rustg_cfg_try_deserialize(...) RUSTG_CALL(RUST_G, "cfg_try_deserialize")()
