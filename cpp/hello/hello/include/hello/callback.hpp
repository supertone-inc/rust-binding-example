#pragma once

#include "hello.h"

#include <functional>
#include <vector>

namespace hello
{
namespace callback
{
template <typename Item = int, typename MappedItem = float, typename Mapper = std::function<MappedItem(Item)>>
std::vector<float> map(const std::vector<Item> &items, const Mapper &mapper)
{
    std::vector<MappedItem> mapped_items(items.size());

    hello__callback__map_with_user_data(
        items.data(),
        mapped_items.data(),
        items.size(),
        (const void *)(&mapper),
        [](const void *user_data, Item item) -> MappedItem
        {
            auto mapper = *(const Mapper *)(user_data);
            return mapper(item);
        }
    );

    return mapped_items;
}
} // namespace callback
} // namespace hello
