#ifndef CONFIG_HPP_
#define CONFIG_HPP_
#include "config.h"
#include "ini.h"
#include <map>
#include <string>

class Config {
  public:
	Config();
	~Config();
	void save();
	bool enabled(int idx);
	void set_enabled(int idx, bool enabled);
	bool multicap();
	void set_multicap(bool enabled);

  private:
	ini_t *ini;
	bool multi = false;
	bool players_enabled[8] = {false, false, false, false, false, false, false, false};
};
#endif // CONFIG_HPP_
