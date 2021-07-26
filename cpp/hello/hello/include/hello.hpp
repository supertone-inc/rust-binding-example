#pragma once

#include "hello.h"
#include "error.hpp"

#include <string>
#include <vector>

namespace hello
{
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
} // namespace hello
