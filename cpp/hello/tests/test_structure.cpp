
#include <catch2/catch.hpp>

#include <hello/structure.hpp>

TEST_CASE("[C] hello__structure__counter")
{
    void *counter = hello__structure__counter__new(1);

    CHECK(hello__structure__counter__increase(counter, 2) == 3);

    hello__structure__counter__delete(counter);
}

TEST_CASE("[C++] hello::structure::Counter")
{
    hello::structure::Counter counter(1);
    REQUIRE(counter.increase(2) == 3);
}
