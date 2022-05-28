#ifndef LOGZ_RS_INCLUDE_BRIDGE_H
#define LOGZ_RS_INCLUDE_BRIDGE_H

#include <stdbool.h>
#include <stdint.h>

///
/// ZEPHYR RTOS
///

// FATAL ERROR HANDLER

__attribute__((__noreturn__)) void logz_rs_error_handler();

// LOG

void log_dbg(const char *restrict msg);

void log_inf(const char *restrict msg);

void log_wrn(const char *restrict msg);

void log_err(const char *restrict msg);

#endif