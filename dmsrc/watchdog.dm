/// Update the watchdog timeout.
/// timeout - time in seconds.
#define rustg_wd_update(timeout) RUSTG_CALL(RUST_G, "wd_update")(isnum(timeout) ? num2text(timeout) : (timeout || "0"))

/// Start the watchdog with the specified timeout.
/// timeout - time in seconds.
#define rustg_wd_start(timeout, webhook_url, message) RUSTG_CALL(RUST_G, "wd_start")(isnum(timeout) ? num2text(timeout) : (timeout || "0"), isnull(webhook_url) ? "" : webhook_url, isnull(message) ? "" : message)

/// Stop the watchdog.
#define rustg_wd_stop(...) RUSTG_CALL(RUST_G, "wd_stop")()
