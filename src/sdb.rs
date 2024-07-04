use std::sync::RwLock;
use surrealdb::{engine::any::Any, opt::auth::Root, Surreal};
use tokio::runtime::Runtime;

static CLIENT: RwLock<Option<DbClient>> = RwLock::new(None);

struct DbClient {
    rt: Runtime,
    connect: Surreal<Any>,
}

fn connect(addr: &str, ns: &str, db: &str, login: &str, pass: &str) -> Result<(), String> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let db = rt.block_on(async move {
        let client = Surreal::init();

        Surreal::<Any>::connect(&client, addr).await.map_err(|err| err.to_string())?;

        if !login.is_empty() && !pass.is_empty() {
            client
                .signin(Root {
                    username: login,
                    password: pass,
                })
                .await
                .map_err(|err| err.to_string())?;
        }

        client
            .use_ns(ns)
            .use_db(db)
            .await
            .map_err(|err| err.to_string())?;

        Ok::<Surreal<Any>, String>(client)
    })?;

    let mut client = CLIENT.try_write().unwrap();

    client.replace(DbClient { rt, connect: db });

    Ok(())
}

fn query(query: &str, binds: &str) -> Result<String, String> {
    let mut client = CLIENT.try_write().unwrap();

    if client.is_none() {
        return Err(String::from("Call 'connect' first"));
    }

    let client = client.as_mut().unwrap();

    client.rt.block_on(async {
        let mut query = client.connect.query(query);

        if !binds.is_empty() {
            let binds: serde_json::Value =
                serde_json::from_str(binds).map_err(|err| err.to_string())?;

            query = query.bind(binds);
        }

        let mut response = query.await.map_err(|err| err.to_string())?;
        let mut result = Vec::with_capacity(response.num_statements());

        for idx in 0..response.num_statements() {
            let response: surrealdb::sql::Value =
                response.take(idx).map_err(|err| err.to_string())?;

            result.push(response.into_json());
        }

        let result = serde_json::to_string(&result).map_err(|err| err.to_string())?;

        Ok::<String, String>(result)
    })
}

fn import(path: &str) -> Result<(), String> {
    let mut client = CLIENT.try_write().unwrap();

    if client.is_none() {
        return Err(String::from("Call 'connect' first"));
    }

    let client = client.as_mut().unwrap();

    client.rt.block_on(async {
        client
            .connect
            .import(path)
            .await
            .map_err(|err| err.to_string())?;

        Ok::<(), String>(())
    })
}

fn export(path: &str) -> Result<(), String> {
    let mut client = CLIENT.try_write().unwrap();

    if client.is_none() {
        return Err(String::from("Call 'connect' first"));
    }

    let client = client.as_mut().unwrap();

    client.rt.block_on(async {
        client
            .connect
            .export(path)
            .await
            .map_err(|err| err.to_string())?;

        Ok::<(), String>(())
    })
}

fn disconnect() {
    CLIENT.try_write().unwrap().take();
}

byond_fn!(fn sdb_connect(addr, ns, db, login, pass) {
    match connect(addr, ns, db, login, pass) {
        Ok(_) => Some(String::new()),
        Err(err) => Some(err)
    }
});

byond_fn!(fn sdb_query(query, binds) {
    match self::query(query, binds) {
        Ok(ret) => Some(ret),
        Err(err) => Some(err)
    }
});

byond_fn!(fn sdb_import(path) {
    match import(path) {
        Ok(_) => Some(String::new()),
        Err(err) => Some(err)
    }
});

byond_fn!(fn sdb_export(path) {
    match export(path) {
        Ok(_) => Some(String::new()),
        Err(err) => Some(err)
    }
});

byond_fn!(
    fn sdb_disconnect() {
        disconnect();

        Some(String::new())
    }
);
