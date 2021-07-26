#pragma once

#include "hello.h"
#include "error.hpp"
#include "string.hpp"

#include <vector>

namespace hello
{
  std::vector<float> concat(const std::vector<float> &a, const std::vector<float> &b)
  {
    std::vector<float> c(a.size() + b.size());
    hello__concat_safe(a.data(), a.size(), b.data(), b.size(), c.data());
    return c;
  }
} // namespace hello
