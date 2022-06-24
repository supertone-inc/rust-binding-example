#include <emscripten/bind.h>

#include <hello_string.hpp>

#include <iostream>

namespace em = emscripten;

EMSCRIPTEN_BINDINGS()
{
    em::function("to_uppercase", &hello::string::to_uppercase);
}
