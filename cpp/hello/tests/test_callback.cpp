#include <catch2/catch.hpp>

#include <callback.hpp>

float mapper(int item)
{
    return item / 2.0f;
}

TEST_CASE("[C] hello__callback__map()")
{
    int items_in[] = {1, 2, 3};
    const size_t item_count = sizeof(items_in) / sizeof(items_in[0]);

    float items_out[item_count];

    hello__callback__map(items_in, items_out, item_count, mapper);

    REQUIRE(items_out[0] == 0.5f);
    REQUIRE(items_out[1] == 1.0f);
    REQUIRE(items_out[2] == 1.5f);
}

TEST_CASE("[C++] hello::callback::map()")
{
    auto result = hello::callback::map({1, 2, 3}, [](int item) { return item / 2.0f; });

    REQUIRE(result == std::vector<float>{0.5f, 1.0f, 1.5f});
}
