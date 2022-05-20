#include <catch2/catch.hpp>

#include <callback.hpp>

float mapper(float item)
{
    return item * 2;
}

TEST_CASE("[C] hello__callback__map_safe()")
{
    float items_in[] = {1.f, 2.f, 3.f};
    const size_t item_count = sizeof(items_in) / sizeof(items_in[0]);

    float items_out[item_count];

    hello__callback__map_safe(items_in, items_out, item_count, mapper);

    REQUIRE(items_out[0] == 2.f);
    REQUIRE(items_out[1] == 4.f);
    REQUIRE(items_out[2] == 6.f);
}

TEST_CASE("[C++] hello::callback::map()")
{
    auto result = hello::callback::map({1.f, 2.f, 3.f}, [](float item) { return item * 2; });

    REQUIRE(result == std::vector<float>{2.f, 4.f, 6.f});
}
