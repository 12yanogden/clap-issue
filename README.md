# clap-issue

## Description

The `only_required_for_required_group` argument should not satisfy the `requires` restriction on the `requires_group`. Only `required_for_dependent_and_required_group` should satify the `requires` restriction for the `requires_group` when `dependent` is specified.

## Steps to Reproduce

1. Clone this repository.
2. Run the tests using the following command:
    ```sh
    cargo test
    ```

## Expected Behavior

The test `test_missing_required_argument_error` should pass, indicating that the `only_required_for_required_group` argument does not satisfy the `requires` restriction on the `requires_group`.

## Actual Behavior

The test `test_missing_required_argument_error` does not pass, indicating that the `only_required_for_required_group` argument incorrectly satisfies the `requires` restriction on the `requires_group`.