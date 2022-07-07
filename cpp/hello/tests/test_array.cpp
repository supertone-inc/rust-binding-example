#include <catch2/catch.hpp>

#include <hello/array.hpp>

TEST_CASE("[C] hello__array__concat_alloc()")
{
    float a[] = {1.f, 2.f};
    size_t a_length = sizeof(a) / sizeof(a[0]);

    float b[] = {3.f, 4.f, 5.f};
    size_t b_length = sizeof(b) / sizeof(b[0]);

    float *array = hello__array__concat_alloc(a, a_length, b, b_length);

    CHECK(array[0] == 1.f);
    CHECK(array[1] == 2.f);
    CHECK(array[2] == 3.f);
    CHECK(array[3] == 4.f);
    CHECK(array[4] == 5.f);

    hello__array__destroy_array(array);
}

TEST_CASE("[C] hello__array__concat()")
{
    float a[] = {1.f, 2.f};
    const size_t a_length = sizeof(a) / sizeof(a[0]);

    float b[] = {3.f, 4.f, 5.f};
    const size_t b_length = sizeof(b) / sizeof(b[0]);

    float c[a_length + b_length];

    hello__array__concat(a, a_length, b, b_length, c);

    REQUIRE(c[0] == 1.f);
    REQUIRE(c[1] == 2.f);
    REQUIRE(c[2] == 3.f);
    REQUIRE(c[3] == 4.f);
    REQUIRE(c[4] == 5.f);
}

TEST_CASE("[C++] hello::array::concat()")
{
    REQUIRE(hello::array::concat({1.f, 2.f}, {3.f, 4.f, 5.f}) == std::vector<float>{1.f, 2.f, 3.f, 4.f, 5.f});
}
