#pragma once

#include "hello.h"

#include <vector>

namespace hello
{
namespace callback
{
std::vector<float> map(const std::vector<float> &items, float (*mapper)(float))
{
    std::vector<float> mapped_items(items.size());
    hello__callback__map_safe(items.data(), mapped_items.data(), items.size(), mapper);
    return mapped_items;
}
} // namespace callback
} // namespace hello
