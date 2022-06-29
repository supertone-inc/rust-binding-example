#include <emscripten/bind.h>

#include <hello_array.hpp>
#include <hello_string.hpp>

#include <iostream>

namespace em = emscripten;

em::val concat(const em::val &a, const em::val &b)
{
    auto a_vec{em::convertJSArrayToNumberVector<float>(a)};
    auto b_vec{em::convertJSArrayToNumberVector<float>(b)};
    auto c_vec{hello::array::concat(a_vec, b_vec)};
    auto c_float32_array{em::typed_memory_view(c_vec.size(), c_vec.data())};

    return em::val::global("Array").call<em::val>("from", c_float32_array);
}

EMSCRIPTEN_BINDINGS()
{
    em::function("to_uppercase", &hello::string::to_uppercase);
    em::function("concat", &concat);
}
