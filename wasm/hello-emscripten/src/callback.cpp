#include <emscripten/bind.h>
#include <hello/callback.hpp>

namespace em = emscripten;

em::val map(const em::val &items, const em::val &mapper)
{
    auto items_vec{em::convertJSArrayToNumberVector<int>(items)};
    auto mapped_items = hello::callback::map(items_vec, [&](int item) { return mapper(item).as<float>(); });

    return em::val{em::typed_memory_view(mapped_items.size(), mapped_items.data())};
}

EMSCRIPTEN_BINDINGS()
{
    em::function("map", &map, em::allow_raw_pointers());
}
