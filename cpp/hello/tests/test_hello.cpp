#include <hello.h>
#include <cstring>
#include <cstdio>

int test_hello(int argc, char* argv[]) {
  hello::print_string("world");

  char* string = hello::get_string("world");

  int result = strcmp(string, "Hello world!");

  hello::destroy_string(string);

  if (result) {
    return -1;
  }

  return 0;
}
