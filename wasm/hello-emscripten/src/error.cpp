#include <emscripten/bind.h>
#include <hello/error.hpp>

namespace em = emscripten;

std::string get_exception_message(intptr_t exception_ptr)
{
    return std::string(reinterpret_cast<std::exception *>(exception_ptr)->what());
}

EMSCRIPTEN_BINDINGS()
{
    em::function("throwError", &hello::error::throw_error);
    em::function("getExceptionMessage", &get_exception_message);
}
