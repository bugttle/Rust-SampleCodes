#include <stdint.h>

////////////////////////////////////////////////////////////////////////////////
// Callbacks from C code to Rust functions

typedef void (*rust_callback1)(int32_t);
rust_callback1 cb1;

int32_t register_callback1(rust_callback1 callback) {
    cb1 = callback;
    return 1;
}

void trigger_callback1() {
    cb1(7); // Will call callback(7) in Rust.
}


////////////////////////////////////////////////////////////////////////////////
// Targeting callbacks to Rust objects

typedef void (*rust_callback2)(void*, int32_t);
void *cb_target;
rust_callback2 cb2;

int32_t register_callback2(void* callback_target, rust_callback2 callback) {
    cb_target = callback_target;
    cb2 = callback;
    return 1;
}

void trigger_callback2() {
    cb2(cb_target, 7); // Will call callback(&rustObject, 7) in Rust.
}
