#pragma once

#include "hello.h"

#include <vector>

namespace hello
{
namespace callback
{
std::vector<float> map(const std::vector<int> &items, float (*mapper)(int))
{
    std::vector<float> mapped_items(items.size());
    hello__callback__map(items.data(), mapped_items.data(), items.size(), mapper);
    return mapped_items;
}
} // namespace callback
} // namespace hello
