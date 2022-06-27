#include <emscripten/bind.h>

#include <hello_string.hpp>
#include <hello_array.hpp>

#include <iostream>

namespace em = emscripten;

const auto concat(const em::val &a, const em::val &b)
{
    const auto vec_a{ em::convertJSArrayToNumberVector<float>(a) };
    const auto vec_b{ em::convertJSArrayToNumberVector<float>(b) };

    const auto vec_c{ hello::array::concat(vec_a, vec_b) };

    return em::val{ em::typed_memory_view(vec_c.size(), vec_c.data()) };
}


EMSCRIPTEN_BINDINGS()
{
    em::function("to_uppercase", &hello::string::to_uppercase);
    em::function("concat", &concat);
}
