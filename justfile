RM_COMMAND := 'trash'

# Default recipy that will list all recipies
default:
    @just --list


# Some tests can create clutter files
# They are suposed to clean them, but in case something goes wrong, this recipy
# will clean that test files
clean:
    - {{RM_COMMAND}} ./test_download_basic_case
    - {{RM_COMMAND}} ./dir_tests
    - {{RM_COMMAND}} ./test_*

# Run all the tests of the project
# We are working with a main crate and multiple lib crates
# So run all of the tests in the project
test:
    - cargo test
    - cd $(pwd)/lib_fileops && cargo test
    - cd $(pwd)/lib_commands && cargo test
