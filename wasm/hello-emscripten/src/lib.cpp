#include <emscripten/bind.h>

#include <hello_array.hpp>
#include <hello_string.hpp>

#include <vector>

namespace em = emscripten;

template <typename T> em::val vector_as_typed_array(const std::vector<T> &vec)
{
    return em::val{em::typed_memory_view(vec.size(), vec.data())};
}

em::val concat(const em::val &a, const em::val &b)
{
    auto a_vec{em::convertJSArrayToNumberVector<float>(a)};
    auto b_vec{em::convertJSArrayToNumberVector<float>(b)};
    auto c_vec{hello::array::concat(a_vec, b_vec)};

    return vector_as_typed_array(c_vec);
}

void concat_preallocated(const std::vector<float> &a, const std::vector<float> &b, std::vector<float> &c)
{
    hello__array__concat(a.data(), a.size(), b.data(), b.size(), c.data());
}

EMSCRIPTEN_BINDINGS()
{
    em::register_vector<float>("Float32Vector")
        .constructor<size_t>()
        .class_function("from", &em::convertJSArrayToNumberVector<float>)
        .function("asTypedArray", &vector_as_typed_array<float>);

    em::function("concat", &concat);
    em::function("concatPreallocated", &concat_preallocated);

    em::function("toUppercase", &hello::string::to_uppercase);
}
