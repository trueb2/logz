#include "logz.h"

#include "autoconf.h"
#include "zephyr.h"
#include "kernel.h"
#include "fatal.h"
#include "init.h"
#include "logging/log.h"

///
/// ZEPHYR RTOS
///

int logz_init(const struct device *_)
{
    switch (CONFIG_LOG_DEFAULT_LEVEL)
    {
    case LOG_LEVEL_ERR:
    {
        logz_init_error();
        break;
    }
    case LOG_LEVEL_WRN:
    {
        logz_init_warn();
        break;
    }
    case LOG_LEVEL_INF:
    {
        logz_init_info();
        break;
    }
    case LOG_LEVEL_DBG:
    {
        logz_init_trace();
        break;
    }
    default:
    {
        logz_init_trace();
        break;
    }
    }
    return 0;
}

SYS_INIT(logz_init, POST_KERNEL, 10);

// FATAL ERROR HANDLER
__attribute__((__noreturn__)) void logz_rs_error_handler()
{
    k_sys_fatal_error_handler(0, NULL);
    NVIC_SystemReset();
}

// LOG

LOG_MODULE_REGISTER(rs, LOG_LEVEL_DBG);

void log_dbg(const char *restrict msg)
{
    LOG_DBG("%s", msg);
}

void log_inf(const char *restrict msg)
{
    LOG_INF("%s", msg);
}

void log_wrn(const char *restrict msg)
{
    LOG_WRN("%s", msg);
}

void log_err(const char *restrict msg)
{
    LOG_ERR("%s", msg);
}
