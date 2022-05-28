#include "bridge.h"
#include "zlog.h"

#include "autoconf.h"
#include "zephyr.h"
#include "kernel.h"
#include "fatal.h"
#include "init.h"
#include "logging/log.h"

///
/// ZEPHYR RTOS
///

void zlog_init(const struct device* _)
{
    switch (CONFIG_LOG_DEFAULT_LEVEL)
    {
    case LOG_LEVEL_ERR:
    {
        zlog_init_error();
        break;
    }
    case LOG_LEVEL_WRN:
    {
        zlog_init_warn();
        break;
    }
    case LOG_LEVEL_INF:
    {
        zlog_init_info();
        break;
    }
    case LOG_LEVEL_DBG:
    {
        zlog_init_trace();
        break;
    }
    default:
    {
        zlog_init_trace();
        break;
    }
    }
}

SYS_INIT(zlog_init, POST_KERNEL, 10);

// FATAL ERROR HANDLER
__attribute__((__noreturn__)) void zlog_rs_error_handler()
{
    k_sys_fatal_error_handler(0, NULL);
    NVIC_SystemReset();
}

// LOG

LOG_MODULE_REGISTER(rs, LOG_LEVEL_DBG);

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
