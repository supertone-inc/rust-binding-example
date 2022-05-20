#include <catch2/catch.hpp>

#include <callback.hpp>

int mapper(int item)
{
    return item * 2;
}

TEST_CASE("[C] hello__callback__map_safe()")
{
    int items_in[] = {1, 2, 3};
    const size_t item_count = sizeof(items_in) / sizeof(items_in[0]);

    int items_out[item_count];

    hello__callback__map_safe(items_in, items_out, item_count, mapper);

    REQUIRE(items_out[0] == 2);
    REQUIRE(items_out[1] == 4);
    REQUIRE(items_out[2] == 6);
}

TEST_CASE("[C++] hello::callback::map()")
{
    auto result = hello::callback::map({1, 2, 3}, [](int item) { return item * 2; });

    REQUIRE(result == std::vector<int>{2, 4, 6});
}
