#ifndef ERROR_HPP_
#define ERROR_HPP_
#include "config.hpp"
#include <tesla.hpp>

class ErrorGui : public tsl::Gui {
  public:
	ErrorGui(std::string msg, Config *cfg, bool can_start = false);
	virtual tsl::elm::Element *createUI() override;
	virtual void update() override;

  private:
	Config *config;
	std::string message;
	bool show_start;
	bool error;
	tsl::elm::ListItem *start_item;
};

#endif // ERROR_HPP_
