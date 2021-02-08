#include <catch2/catch.hpp>

#include <hello.hpp>
#include <iostream>

TEST_CASE("test_cpp_to_uppercase", "cpp")
{
  REQUIRE(hello::to_uppercase(std::string("Hello World!")) == "HELLO WORLD!");
}

TEST_CASE("test_cpp_concat", "cpp")
{
  REQUIRE(hello::concat({1.f, 2.f}, {3.f, 4.f, 5.f}) == std::vector<float>{1.f, 2.f, 3.f, 4.f, 5.f});
}

TEST_CASE("test_cpp_raise_error", "cpp")
{
  REQUIRE_THROWS_AS(hello::raise_error(), hello::Exception);
  REQUIRE_THROWS_WITH(hello::raise_error(), "error raised from Rust!");
}
