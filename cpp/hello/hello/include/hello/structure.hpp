#pragma once

#include "hello.h"

namespace hello
{
namespace structure
{
class Counter
{
  public:
    Counter(int count) : raw(hello__structure__counter__new(count))
    {
    }

    virtual ~Counter()
    {
        hello__structure__counter__delete(raw);
    }

    int increase(int amount)
    {
        return hello__structure__counter__increase(raw, amount);
    }

  private:
    void *raw;
};
} // namespace structure
} // namespace hello
