#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include <hello.h>

TEST_CASE("test_to_uppercase", "[hello]") {
  char* string = hello::to_uppercase("Hello World!");

  CHECK(std::string(string) == "HELLO WORLD!");

  hello::destroy_string(string);
}
