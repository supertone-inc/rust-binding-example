#pragma once

#include "hello.h"

#include <exception>
#include <string>

namespace hello
{
namespace error
{
class Exception : public std::exception
{
public:
    Exception(const std::string &message)
        : message(message){};

    virtual const char *what() const throw()
    {
        return message.c_str();
    }

private:
    std::string message;
};

void throw_error()
{
    int result = hello__error__throw_error();
    if (!result)
    {
        return;
    }

    size_t error_length = hello__error__last_error_length();
    if (!error_length)
    {
        return;
    }

    std::string message(error_length, '\0');
    int message_length = hello__error__last_error_message((char *)message.c_str(), message.capacity());
    if (message_length <= 0)
    {
        throw Exception("Fetching error message failed");
    }

    throw Exception(message);
}
} // namespace error
} // namespace hello
