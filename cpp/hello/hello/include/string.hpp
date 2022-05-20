#pragma once

#include "hello.h"

#include <string>

namespace hello
{
namespace string
{
std::string to_uppercase(const std::string &in_string)
{
    std::string out_string(in_string.length(), '\0');
    hello__string__to_uppercase(in_string.c_str(), (char *)out_string.c_str());
    return out_string;
}
} // namespace string
} // namespace hello
