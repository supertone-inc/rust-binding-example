#include <catch2/catch.hpp>
#include <hello/error.hpp>

TEST_CASE("[C] hello__error__throw_error()")
{

    {
        int result = hello__error__throw_error();
        REQUIRE(result < 0);

        size_t message_length = hello__error__get_last_error_message_length();
        REQUIRE(message_length > 0);

        char *message = new char[message_length + 1];
        size_t copied_length = hello__error__get_last_error_message(message, message_length + 1);
        CHECK(copied_length == message_length);
        CHECK(std::string(message) == "error from Rust!");

        delete[] message;
    }

    {
        int result = hello__error__throw_error();
        REQUIRE(result < 0);

        size_t message_length = hello__error__get_last_error_message_length();
        REQUIRE(message_length > 0);

        std::string message(message_length, '\0');
        size_t copied_length = hello__error__get_last_error_message((char *)message.data(), message.length());
        REQUIRE(copied_length == message.length());
        REQUIRE(message == "error from Rust!");
    }

    {
        int result = hello__error__throw_error();
        REQUIRE(result < 0);

        size_t message_length = hello__error__get_last_error_message_length();
        REQUIRE(message_length > 0);

        std::string message(message_length - 1, '\0');
        size_t copied_length = hello__error__get_last_error_message((char *)message.data(), message.length());
        REQUIRE(copied_length == message.length());
        REQUIRE(message == "error from Rust");
    }
}

TEST_CASE("[C++] hello::error::throw_error()")
{
    REQUIRE_THROWS_MATCHES(
        []() { hello::error::throw_error(); }(),
        hello::error::Error,
        Catch::Message("error from Rust!")
    );
}
