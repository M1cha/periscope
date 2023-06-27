#ifndef ERROR_HPP_
#define ERROR_HPP_
#include <tesla.hpp>

class ErrorGui : public tsl::Gui {
  public:
	ErrorGui(std::string msg);
	virtual tsl::elm::Element *createUI() override;

  private:
	std::string message;
};

#endif // ERROR_HPP_
