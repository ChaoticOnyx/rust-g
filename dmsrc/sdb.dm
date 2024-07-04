/// addr - A folder path or an IP address.
/// ns - Namespace.
/// db - Database name.
/// login - Root login. Optional.
/// pass - Root password. Optional.
#define rustg_sdb_connect(addr, ns, db, login, pass) RUSTG_CALL(RUST_G, "sdb_connect")(addr, ns, db, isnull(login) ? "" : login, isnull(pass) ? "" : pass)

/// Executes a query.
/// query - The query itself.
/// binds - a JSON encoded string, for example: `"{ \"some_arg\": \"Value\" }"`. Optional.
#define rustg_sdb_query(query, binds) RUSTG_CALL(RUST_G, "sdb_query")(query, isnull(binds) ? "" : binds)

/// Import an SQL file.
/// path - Path to the SQL file.
#define rustg_sdb_import(path) RUSTG_CALL(RUST_G, "sdb_import")(path)

/// Dumps the database into the SQL file.
/// path - Path to the file where to save the dump.
#define rustg_sdb_export(path) RUSTG_CALL(RUST_G, "sdb_export")(path)

#define rustg_sdb_disconnect(...) RUSTG_CALL(RUST_G, "sdb_disconnect")()
