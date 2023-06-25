#include "config.hpp"
#include <map>
#include <stdio.h>
#include <string>
#include <sys/stat.h>
#include <sys/types.h>
#include <tesla.hpp>

Config::Config() {
	FILE *f = fopen(CONFIG_PATH, "r");
	char buf[256] = {0};
	if (f != NULL) {
		fread(buf, sizeof(buf), 1, f);
		fclose(f);
		std::string s = std::string(buf);
		raw = tsl::hlp::ini::parseIni(s);
		for (auto &i : raw["players"]) {
			int index = i.first[0] - '1';
			if (i.second.find("true") != std::string::npos) {
				players_enabled[index] = true;
			} else {
				players_enabled[index] = false;
			}
		}
	}
}

void Config::save() {
	struct stat st = {0};
	if (stat(CONFIG_DIR, &st) == -1) {
		mkdir(CONFIG_DIR, 0700);
	}
	FILE *f = fopen(CONFIG_PATH, "w");
	std::string s = tsl::hlp::ini::unparseIni(raw).c_str();
	fwrite(s.c_str(), s.size(), 1, f);
	fclose(f);
}

bool Config::enabled(int idx) {
	return players_enabled[idx];
}

void Config::set_enabled(int idx, bool enabled) {
	players_enabled[idx] = enabled;
	char key[] = "1";
	key[0] += idx;
	raw["players"][key] = enabled ? "true" : "false";
	save();
}

