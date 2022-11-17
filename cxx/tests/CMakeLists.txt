project(skywhale.tests)

# GoogleTest requires at least C++14

include(FetchContent)
FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG release-1.12.1
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../skywhale/include)

file(GLOB_RECURSE TESTS ${CMAKE_CURRENT_SOURCE_DIR}/*.cc )

enable_testing()

add_executable(skywhale.tests ${TESTS})
target_link_libraries(skywhale.tests skywhale GTest::gtest_main)

add_test(tests skywhale.tests)

include(GoogleTest)

#gtest_discover_tests(skywhale.tests)

#add_custom_command(TARGET 
#  skywhale.tests 
#  POST_BUILD COMMAND 
#  ctest -C $<CONFIGURATION> --verbose 
#  --output-on-failure )