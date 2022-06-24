#include <emscripten/bind.h>

#include <iostream>

namespace embind = emscripten;

void greet()
{
    std::cout << "hello" << std::endl;
}

EMSCRIPTEN_BINDINGS()
{
    embind::function("greet", &greet);
}
