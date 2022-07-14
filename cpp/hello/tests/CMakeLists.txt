cmake_minimum_required(VERSION 3.20)

enable_testing()

project(tests)

include(FetchContent)
FetchContent_Declare(
  Catch2
  GIT_REPOSITORY  https://github.com/catchorg/Catch2.git
  GIT_TAG         v2.13.6
  GIT_SHALLOW     TRUE
)
FetchContent_MakeAvailable(Catch2)

add_executable(${PROJECT_NAME}
  main.cpp
  test_array.cpp
  test_callback.cpp
  test_error.cpp
  test_string.cpp
)

target_include_directories(${PROJECT_NAME} PRIVATE ${HELLO_DIR}/include)
target_link_directories(${PROJECT_NAME} PRIVATE ${HELLO_DIR}/lib)
target_link_libraries(${PROJECT_NAME} Catch2::Catch2 hello_cpp)

if(HELLO_STATIC)
  if(UNIX AND NOT APPLE)
    target_link_libraries(${PROJECT_NAME} dl pthread)
  elseif(WIN32)
    target_link_libraries(${PROJECT_NAME} bcrypt userenv ws2_32)
  endif()
endif()

if(WIN32)
  file(GLOB_RECURSE DYNAMIC_LIBS "${HELLO_DIR}/lib/*.dll")
	foreach(dll ${DYNAMIC_LIBS})
		add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
			COMMAND ${CMAKE_COMMAND} -E copy ${dll} $<TARGET_FILE_DIR:${PROJECT_NAME}>
    )
	endforeach()
endif()

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/contrib)
include(CTest)
include(Catch)
catch_discover_tests(${PROJECT_NAME})
