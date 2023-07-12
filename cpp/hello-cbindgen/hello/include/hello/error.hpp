#pragma once

#include "hello.h"

#include <stdexcept>
#include <string>

namespace hello
{
namespace error
{
class Error : public std::runtime_error
{
    using std::runtime_error::runtime_error;
};

void check_error(int error)
{
    if (!error)
    {
        return;
    }

    size_t message_length = hello__error__get_last_error_message_length();
    std::string message(message_length, '\0');
    hello__error__get_last_error_message((char *)message.data(), message.length());

    throw Error(message);
}

void throw_error()
{
    check_error(hello__error__throw_error());
}
} // namespace error
} // namespace hello
