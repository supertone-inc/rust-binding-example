#pragma once

#include "hello.h"

#include <vector>

namespace hello
{
namespace array
{
std::vector<float> concat(const std::vector<float> &a, const std::vector<float> &b)
{
    std::vector<float> c(a.size() + b.size());
    hello__array__concat_safe(a.data(), a.size(), b.data(), b.size(), c.data());
    return c;
}
} // namespace array
} // namespace hello
