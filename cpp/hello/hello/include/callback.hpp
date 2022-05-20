#pragma once

#include "hello.h"

#include <vector>

namespace hello
{
namespace callback
{
std::vector<int> map(const std::vector<int> &items, int (*mapper)(int))
{
    std::vector<int> mapped_items(items.size());
    hello__callback__map_safe(items.data(), mapped_items.data(), items.size(), mapper);
    return mapped_items;
}
} // namespace callback
} // namespace hello
