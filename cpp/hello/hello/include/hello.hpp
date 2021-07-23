#pragma once

#include "hello.h"

#include <string>
#include <vector>
#include <exception>

namespace hello
{
  class Exception : public std::exception
  {
  public:
    Exception(const std::string &message) : message(message){};

    virtual const char *what() const throw()
    {
      return message.c_str();
    }

  private:
    std::string message;
  };

  std::string to_uppercase(const std::string &in_string)
  {
    std::string out_string(in_string.length(), '\0');
    hello__to_uppercase_safe(in_string.c_str(), (char *)out_string.c_str());
    return out_string;
  }

  std::vector<float> concat(const std::vector<float> &a, const std::vector<float> &b)
  {
    std::vector<float> c(a.size() + b.size());
    hello__concat_safe(a.data(), a.size(), b.data(), b.size(), c.data());
    return c;
  }

  void raise_error()
  {
    int result = hello__raise_error();
    if (!result)
    {
      return;
    }

    size_t error_length = hello__last_error_length();
    if (!error_length)
    {
      return;
    }

    std::string message(error_length, '\0');
    int message_length = hello__last_error_message((char *)message.c_str(), message.capacity());
    if (message_length <= 0)
    {
      throw Exception("Fetching error message failed");
    }

    throw Exception(message);
  }
} // namespace hello
