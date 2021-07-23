#include <catch2/catch.hpp>

#include <hello.h>

TEST_CASE("[C] to_uppercase()")
{
  char *string = hello__to_uppercase("Hello World!");

  CHECK(std::string(string) == "HELLO WORLD!");

  hello__destroy_string(string);
}

TEST_CASE("[C] to_uppercase_safe()")
{
  std::string in_string("Hello World!");
  std::string out_string(in_string.length(), '\0');

  hello__to_uppercase_safe(in_string.c_str(), (char *)out_string.c_str());

  REQUIRE(out_string == "HELLO WORLD!");
}

TEST_CASE("[C] concat()")
{
  float a[] = {1.f, 2.f};
  size_t a_length = sizeof(a) / sizeof(a[0]);

  float b[] = {3.f, 4.f, 5.f};
  size_t b_length = sizeof(b) / sizeof(b[0]);

  float *array = hello__concat(a, a_length, b, b_length);

  CHECK(array[0] == 1.f);
  CHECK(array[1] == 2.f);
  CHECK(array[2] == 3.f);
  CHECK(array[3] == 4.f);
  CHECK(array[4] == 5.f);

  hello__destroy_array(array);
}

TEST_CASE("[C] concat_safe()")
{
  float a[] = {1.f, 2.f};
  const size_t a_length = sizeof(a) / sizeof(a[0]);

  float b[] = {3.f, 4.f, 5.f};
  const size_t b_length = sizeof(b) / sizeof(b[0]);

  float c[a_length + b_length];

  hello__concat_safe(a, a_length, b, b_length, c);

  REQUIRE(c[0] == 1.f);
  REQUIRE(c[1] == 2.f);
  REQUIRE(c[2] == 3.f);
  REQUIRE(c[3] == 4.f);
  REQUIRE(c[4] == 5.f);
}

TEST_CASE("[C] raise_error()")
{
  int result = hello__raise_error();

  REQUIRE(result < 0);

  size_t error_length = hello__last_error_length();

  REQUIRE(error_length > 0);

  std::string message(error_length - 1, '\0');
  int message_length = hello__last_error_message((char *)message.c_str(), message.capacity());

  REQUIRE(message_length > 0);
  REQUIRE(message == "error raised from Rust!");
}
