#include <emscripten/bind.h>

#include <hello/string.hpp>

namespace em = emscripten;

EMSCRIPTEN_BINDINGS()
{
    em::function("toUppercase", &hello::string::to_uppercase);
}
