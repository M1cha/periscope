#ifndef SCOPE_CONFIG_H_
#define SCOPE_CONFIG_H_
#include "ini.h"
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

ini_t *config_load();
bool config_player_enabled(ini_t *ini, int idx);
void config_enable_player(ini_t *ini, int idx, bool enable);
void config_save(ini_t *ini);

#ifdef __cplusplus
}
#endif

#define CONFIG_PATH "/config/sys-scope/config.ini"
#define CONFIG_DIR  "/config/sys-scope"

#endif // SCOPE_CONFIG_H_