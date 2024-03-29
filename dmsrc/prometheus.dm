#define rustg_prom_init(port) RUSTG_CALL(RUST_G, "prom_init")(istext(port) ? port : num2text(port))

#define rustg_prom_set_labels(labels) RUSTG_CALL(RUST_G, "prom_set_labels")(json_encode(labels))

// Counters

#define rustg_prom_counter_register(id, desc) RUSTG_CALL(RUST_G, "prom_counter_register")(id, desc)

#define rustg_prom_counter_inc(id, labels) RUSTG_CALL(RUST_G, "prom_counter_inc")(id, json_encode(labels))

#define rustg_prom_counter_inc_by(id, value, labels) RUSTG_CALL(RUST_G, "prom_counter_inc_by")(id, istext(value) ? value : num2text(value), json_encode(labels))

// Integer gauges

#define rustg_prom_gauge_int_register(id, desc) RUSTG_CALL(RUST_G, "prom_gauge_int_register")(id, desc)

#define rustg_prom_gauge_int_inc(id, labels) RUSTG_CALL(RUST_G, "prom_gauge_int_inc")(id, json_encode(labels))

#define rustg_prom_gauge_int_inc_by(id, value, labels) RUSTG_CALL(RUST_G, "prom_gauge_int_inc_by")(id, istext(value) ? value : num2text(value), json_encode(labels))

#define rustg_prom_gauge_int_dec(id, labels) RUSTG_CALL(RUST_G, "prom_gauge_int_dec")(id, json_encode(labels))

#define rustg_prom_gauge_int_dec_by(id, value, labels) RUSTG_CALL(RUST_G, "prom_gauge_int_dec_by")(id, istext(value) ? value : num2text(value), json_encode(labels))

#define rustg_prom_gauge_int_set(id, value, labels) RUSTG_CALL(RUST_G, "prom_gauge_int_set")(id, istext(value) ? value : num2text(value), json_encode(labels))

// Float gauges

#define rustg_prom_gauge_float_register(id, desc) RUSTG_CALL(RUST_G, "prom_gauge_float_register")(id, desc)

#define rustg_prom_gauge_float_inc(id, labels) RUSTG_CALL(RUST_G, "prom_gauge_float_inc")(id, json_encode(labels))

#define rustg_prom_gauge_float_inc_by(id, value, labels) RUSTG_CALL(RUST_G, "prom_gauge_float_inc_by")(id, istext(value) ? value : num2text(value), json_encode(labels))

#define rustg_prom_gauge_float_dec(id, labels) RUSTG_CALL(RUST_G, "prom_gauge_float_dec")(id), json_encode(labels)

#define rustg_prom_gauge_float_dec_by(id, value, labels) RUSTG_CALL(RUST_G, "prom_gauge_float_dec_by")(id, istext(value) ? value : num2text(value), json_encode(labels))

#define rustg_prom_gauge_float_set(id, value, labels) RUSTG_CALL(RUST_G, "prom_gauge_float_set")(id, istext(value) ? value : num2text(value), json_encode(labels))
