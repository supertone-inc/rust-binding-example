project(tests)

Include(FetchContent)
FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v2.13.1)
FetchContent_MakeAvailable(Catch2)

include_directories(${CMAKE_SOURCE_DIR}/hello/include)
link_directories(${CMAKE_SOURCE_DIR}/hello/lib)

add_executable(${PROJECT_NAME} main.cpp test_hello_c.cpp test_hello_cpp.cpp)
target_link_libraries(${PROJECT_NAME} Catch2::Catch2 hello)

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/contrib)
include(CTest)
include(Catch)
catch_discover_tests(${PROJECT_NAME})
