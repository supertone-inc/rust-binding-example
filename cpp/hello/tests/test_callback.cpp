#include <catch2/catch.hpp>

#include <hello/callback.hpp>

float mapper(int item)
{
    return item / 2.0f;
}

TEST_CASE("[C] hello__callback__map()")
{
    int in_items[] = {1, 2, 3};
    const size_t item_count = sizeof(in_items) / sizeof(in_items[0]);

    float out_items[item_count];

    hello__callback__map(in_items, out_items, item_count, mapper);

    REQUIRE(out_items[0] == 0.5f);
    REQUIRE(out_items[1] == 1.0f);
    REQUIRE(out_items[2] == 1.5f);
}

TEST_CASE("[C++] hello::callback::map()")
{
    auto result = hello::callback::map({1, 2, 3}, [](int item) { return item / 2.0f; });

    REQUIRE(result == std::vector<float>{0.5f, 1.0f, 1.5f});
}
