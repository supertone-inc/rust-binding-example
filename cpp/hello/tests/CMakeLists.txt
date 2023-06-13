cmake_minimum_required(VERSION 3.20)
project(tests)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)
FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG v2.13.6
  GIT_SHALLOW TRUE
)
FetchContent_MakeAvailable(Catch2)

add_executable(${PROJECT_NAME}
  main.cpp
  test_array.cpp
  test_callback.cpp
  test_error.cpp
  test_string.cpp
  test_structure.cpp
)
target_include_directories(${PROJECT_NAME} PRIVATE ../hello/include)
target_link_libraries(${PROJECT_NAME} Catch2::Catch2)

if(HELLO_STATIC)
  target_link_libraries(${PROJECT_NAME} ${CMAKE_CURRENT_LIST_DIR}/../hello/lib/${CMAKE_STATIC_LIBRARY_PREFIX}hello_cpp${CMAKE_STATIC_LIBRARY_SUFFIX})

  if(UNIX AND NOT APPLE)
    target_link_libraries(${PROJECT_NAME} dl pthread)
  elseif(WIN32)
    target_link_libraries(${PROJECT_NAME} bcrypt userenv ws2_32)
  endif()
else()
  if(UNIX)
    target_link_libraries(${PROJECT_NAME} ${CMAKE_CURRENT_LIST_DIR}/../hello/lib/${CMAKE_SHARED_LIBRARY_PREFIX}hello_cpp${CMAKE_SHARED_LIBRARY_SUFFIX})
  else()
    target_link_libraries(${PROJECT_NAME} ${CMAKE_CURRENT_LIST_DIR}/../hello/lib/hello_cpp.dll.lib)

    file(GLOB_RECURSE dlls ../hello/lib/*.dll)

    foreach(dll ${dlls})
      add_custom_command(
        TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy ${dll} $<TARGET_FILE_DIR:${PROJECT_NAME}>
      )
    endforeach()
  endif()
endif()

add_custom_target(
  run_${PROJECT_NAME}
  ALL
  COMMAND ${PROJECT_NAME}
  DEPENDS ${PROJECT_NAME}
)
