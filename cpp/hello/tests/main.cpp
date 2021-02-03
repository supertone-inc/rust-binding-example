#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include <hello.h>

TEST_CASE("test_to_uppercase", "[hello]") {
  char* string = hello::to_uppercase("Hello World!");

  CHECK(std::string(string) == "HELLO WORLD!");

  hello::destroy_string(string);
}

TEST_CASE("test_concat", "[hello]") {
  float a[] = {1.f, 2.f};
  float b[] = {3.f, 4.f, 5.f};
  float* array = hello::concat(a, 2, b, 3);

  CHECK(array[0] == 1.f);
  CHECK(array[1] == 2.f);
  CHECK(array[2] == 3.f);
  CHECK(array[3] == 4.f);
  CHECK(array[4] == 5.f);

  hello::destroy_array(array);
}
