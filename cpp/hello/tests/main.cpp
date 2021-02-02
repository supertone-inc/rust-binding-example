#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include <hello.h>

TEST_CASE("print_string", "[hello]") {
  hello::print_string("world");
}

TEST_CASE("get_string", "[hello]") {
  char* string = hello::get_string("world");

  CHECK(std::string(string) == "Hello world!");

  hello::destroy_string(string);
}
