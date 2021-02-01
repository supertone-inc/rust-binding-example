project(hello_test)

set(INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../hello/include)
set(LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../hello/lib)

include_directories(${INCLUDE_DIR})
link_directories(${LIB_DIR})

find_library(hello
  NAMES hello
  HINTS ${LIB_DIR}
)

set(TEST_SOURCES
  test_hello.cpp
)

create_test_sourcelist(TESTS
  tests.cpp
  ${TEST_SOURCES}
)

add_executable(${PROJECT_NAME} ${TESTS})

target_link_libraries(${PROJECT_NAME} hello)

foreach(TEST_SOURCE ${TEST_SOURCES})
  get_filename_component(TEST_NAME ${TEST_SOURCE} NAME_WE)
  add_test(${TEST_NAME} ${PROJECT_NAME} ${TEST_NAME})
endforeach(TEST_SOURCE)
