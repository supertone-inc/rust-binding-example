#include <catch2/catch.hpp>

#include <error.hpp>

TEST_CASE("[C] hello__error__throw_error()")
{
  int result = hello__error__throw_error();
  REQUIRE(result < 0);

  size_t error_length = hello__error__last_error_length();
  REQUIRE(error_length > 0);

  char *message = new char[error_length];
  int message_length = hello__error__last_error_message(message, error_length);
  CHECK(message_length == error_length - 1);
  CHECK(std::string(message) == "error from Rust!");

  delete[] message;
}

TEST_CASE("[C++] hello::error::throw_error()")
{
  REQUIRE_THROWS_AS(hello::error::throw_error(), hello::error::Exception);
  REQUIRE_THROWS_WITH(hello::error::throw_error(), "error from Rust!");
}
