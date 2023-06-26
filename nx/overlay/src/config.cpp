#include "config.hpp"
#include "config.h"
#include <map>
#include <stdio.h>
#include <string>
#include <sys/stat.h>
#include <sys/types.h>
#include <tesla.hpp>

Config::Config() {
	ini = config_load();
	for (int i = 0; i < 8; i++) {
		players_enabled[i] = config_player_enabled(ini, i);
	}
	multi = config_multicap_enabled(ini);
}

void Config::save() {
	config_save(ini);
}

bool Config::enabled(int idx) {
	return players_enabled[idx];
}

void Config::set_enabled(int idx, bool enabled) {
	players_enabled[idx] = enabled;
	config_enable_player(ini, idx, enabled);
	if (enabled && !multi) {
		for (int i = 0; i < 8; i++) {
			if (i != idx) {
				players_enabled[i] = false;
				config_enable_player(ini, i, false);
			}
		}
	}
	save();
}

bool Config::multicap() {
	return multi;
}

void Config::set_multicap(bool enabled) {
	multi = enabled;
	config_enable_multicap(ini, enabled);
	bool disabling = false;
	for (int i = 0; i < 8; i++) {
		if (disabling) {
			players_enabled[i] = false;
			config_enable_player(ini, i, false);
		} else if (players_enabled[i]) {
			disabling = true;
		}
	}
	save();
}
