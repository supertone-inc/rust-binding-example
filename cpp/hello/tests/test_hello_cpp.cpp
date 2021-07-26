#include <catch2/catch.hpp>

#include <hello.hpp>
#include <iostream>

TEST_CASE("[C++] hello::array::concat()")
{
  REQUIRE(hello::array::concat({1.f, 2.f}, {3.f, 4.f, 5.f}) == std::vector<float>{1.f, 2.f, 3.f, 4.f, 5.f});
}

TEST_CASE("[C++] hello::string::to_uppercase()")
{
  REQUIRE(hello::string::to_uppercase(std::string("Hello World!")) == "HELLO WORLD!");
}

TEST_CASE("[C++] hello::error::raise_error()")
{
  REQUIRE_THROWS_AS(hello::error::raise_error(), hello::error::Exception);
  REQUIRE_THROWS_WITH(hello::error::raise_error(), "error raised from Rust!");
}
