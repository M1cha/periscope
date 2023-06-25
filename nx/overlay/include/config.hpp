#ifndef CONFIG_HPP_
#define CONFIG_HPP_
#include <map>
#include <string>

class Config {
  public:
	Config();
	void save();
	bool enabled(int idx);
	void set_enabled(int idx, bool enabled);

  private:
	bool players_enabled[8] = {false, false, false, false, false, false, false, false};
	std::map<std::string, std::map<std::string, std::string>> raw = {
	    {"players", {{"1", "true"}, {"2", "false"}, {"3", "false"}, {"4", "false"}, {"5", "false"}, {"6", "false"}, {"7", "false"}, {"8", "false"}}}};
};

#define CONFIG_DIR  "/config/sys-scope/"
#define CONFIG_PATH "/config/sys-scope/config.ini"

#endif // CONFIG_HPP_
