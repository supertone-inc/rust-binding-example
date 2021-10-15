project(tests)

include(FetchContent)
FetchContent_Declare(
  Catch2
  GIT_REPOSITORY  https://github.com/catchorg/Catch2.git
  GIT_TAG         v2.13.6
  GIT_SHALLOW     TRUE
)
FetchContent_MakeAvailable(Catch2)

include_directories(${CMAKE_SOURCE_DIR}/hello/include)
link_directories(${CMAKE_SOURCE_DIR}/hello/lib)

add_executable(${PROJECT_NAME}
  main.cpp
  test_array.cpp
  test_error.cpp
  test_string.cpp
)
target_link_libraries(${PROJECT_NAME} Catch2::Catch2 hello_cpp)

if(WIN32)
  file(GLOB_RECURSE DYNAMIC_LIBS "${CMAKE_SOURCE_DIR}/hello/lib/*.dll")
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
