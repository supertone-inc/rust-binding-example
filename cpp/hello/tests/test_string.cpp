#include <catch2/catch.hpp>
#include <hello/string.hpp>

TEST_CASE("[C] hello__string__to_uppercase_alloc()")
{
    char *string = hello__string__to_uppercase_alloc("Hello World!");

    CHECK(std::string(string) == "HELLO WORLD!");

    hello__string__destroy_string(string);
}

TEST_CASE("[C] hello__string__to_uppercase()")
{
    std::string in_string("Hello World!");
    std::string out_string(in_string.length(), '\0');

    hello__string__to_uppercase(in_string.c_str(), (char *)out_string.c_str());

    REQUIRE(out_string == "HELLO WORLD!");
}

TEST_CASE("[C++] hello::string::to_uppercase()")
{
    REQUIRE(hello::string::to_uppercase(std::string("Hello World!")) == "HELLO WORLD!");
}
