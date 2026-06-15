# Testing Framework Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Clean up the existing eshu integration tests by refactoring `tests/parser_tests.rs` into submodules and expand test coverage for edge cases.

**Architecture:** Split the long `tests/parser_tests.rs` file into modules inside `tests/parser_tests/`. Add new tests for key-value parsing, `XffValue` casting, subcommands, and builder validation.

**Tech Stack:** Rust (standard library, test framework), `eshu`, `athena`, `nemesis`.

---

### Task 1: Module Entry Point Configuration

**Files:**
- Modify: `tests/parser_tests.rs`
- Create: `tests/parser_tests/basic.rs`
- Create: `tests/parser_tests/end_of_flag.rs`
- Create: `tests/parser_tests/errors.rs`
- Create: `tests/parser_tests/key_value.rs`
- Create: `tests/parser_tests/regression.rs`
- Create: `tests/parser_tests/store.rs`
- Create: `tests/parser_tests/xff.rs`

- [x] **Step 1: Replace entry point `tests/parser_tests.rs`**
  Modify `tests/parser_tests.rs` to contain only the submodule declarations.
  ```rust
  mod basic;
  mod end_of_flag;
  mod errors;
  mod key_value;
  mod regression;
  mod store;
  mod xff;
  ```

- [x] **Step 2: Create empty module files**
  Create empty files for all seven submodules under the directory `tests/parser_tests/`.

- [x] **Step 3: Verify build and test status**
  Run: `cargo test --test parser_tests`
  Expected: Passes with 0 tests run (as all modules are currently empty).

- [x] **Step 4: Commit**
  Run:
  ```bash
  git add tests/parser_tests.rs tests/parser_tests/
  git commit -m "refactor(tests): partition parser_tests into submodules"
  ```

---

### Task 2: Migrate Basic Parser Tests

**Files:**
- Modify: `tests/parser_tests/basic.rs`

- [x] **Step 1: Move basic tests into `tests/parser_tests/basic.rs`**
  Add the migrated tests to `tests/parser_tests/basic.rs`:
  ```rust
  use eshu::{Cli, CliFlag};

  #[test]
  fn test_long_flag_basic() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("a-flag")
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec!["test".to_string(), "--a-flag".to_string()]);
      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("a-flag"));
  }

  #[test]
  fn partial_flags() {
      use eshu::{StoreSyntax, StoreType};
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("port-other")
                  .with_flag_char('o')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("boolean")
                  .with_flag_char('b')
                  .with_about("Boolean", "Testing bool")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--port-oth".to_string(),
              "8080".to_string(),
              "--port".to_string(),
              "0123".to_string(),
              "--bo".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("port"));
      assert_eq!(
          cli.get_flag_store("port").unwrap().as_value().unwrap(),
          &vec!["0123".to_string()]
      );
      assert!(cli.is_flag_entered("port-other"));
      assert_eq!(
          cli.get_flag_store("port-other")
              .unwrap()
              .as_value()
              .unwrap(),
          &vec!["8080".to_string()]
      );
      assert!(cli.is_flag_entered("boolean"));
  }
  ```

- [x] **Step 2: Run basic tests to verify they pass**
  Run: `cargo test --test parser_tests basic`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/parser_tests/basic.rs
  git commit -m "refactor(tests): migrate basic parser tests"
  ```

---

### Task 3: Migrate End of Flag Marker Tests

**Files:**
- Modify: `tests/parser_tests/end_of_flag.rs`

- [x] **Step 1: Move end of flag tests into `tests/parser_tests/end_of_flag.rs`**
  Add the migrated tests:
  ```rust
  use eshu::{Cli, CliFlag};

  #[test]
  fn end_of_flag_marker_no_store_long_flag() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("a-flag")
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--a-flag".to_string(),
              "--".to_string(),
              "RougeArgument".to_string(),
          ]);
      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("a-flag"));
      assert_eq!(
          cli.get_stray_positional_args(),
          &vec!["RougeArgument".to_string()]
      );
  }

  #[test]
  fn end_of_flag_marker_no_store_short_flag() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("a-flag")
                  .with_flag_char('a')
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "-a".to_string(),
              "--".to_string(),
              "RougeArgument".to_string(),
          ]);
      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("a-flag"));
      assert_eq!(
          cli.get_stray_positional_args(),
          &vec!["RougeArgument".to_string()]
      );
  }

  #[test]
  fn end_of_flag_marker_no_store_group_flag() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("a-flag")
                  .with_flag_char('a')
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("b-flag")
                  .with_flag_char('b')
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "-ba".to_string(),
              "--".to_string(),
              "RougeArgument".to_string(),
          ]);
      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("a-flag"));
      assert!(cli.is_flag_entered("b-flag"));
      assert_eq!(
          cli.get_stray_positional_args(),
          &vec!["RougeArgument".to_string()]
      );
  }
  ```

- [x] **Step 2: Run end-of-flag tests to verify they pass**
  Run: `cargo test --test parser_tests end_of_flag`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/parser_tests/end_of_flag.rs
  git commit -m "refactor(tests): migrate end of flag tests"
  ```

---

### Task 4: Migrate Store Tests

**Files:**
- Modify: `tests/parser_tests/store.rs`

- [x] **Step 1: Move store tests into `tests/parser_tests/store.rs`**
  Add the migrated tests:
  ```rust
  use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

  #[test]
  fn detached_store() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("a-flag")
                  .with_flag_char('a')
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .with_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("boolean")
                  .with_flag_char('b')
                  .with_about("Boolean", "Testing bool")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "-a".to_string(),
              "detachedValue".to_string(),
              "-p".to_string(),
              "8080".to_string(),
              "--port".to_string(),
              "0420".to_string(),
              "-p".to_string(),
              "8081".to_string(),
              "-bp".to_string(),
              "1234".to_string(),
              "-a".to_string(),
              "--port".to_string(),
              "5678".to_string(),
              "-bap".to_string(),
              "9510".to_string(),
          ]);
      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("a-flag"));
      assert_eq!(
          cli.get_flag_store("a-flag").unwrap().as_value().unwrap(),
          &vec!["detachedValue".to_string()]
      );
      assert!(cli.is_flag_entered("boolean"));
      assert!(cli.get_flag_store("boolean").unwrap().exists());
      assert!(cli.is_flag_entered("port"));
      assert_eq!(
          cli.get_flag_store("port").unwrap().as_value().unwrap(),
          &vec![
              "8080".to_string(),
              "0420".to_string(),
              "8081".to_string(),
              "1234".to_string(),
              "5678".to_string(),
              "9510".to_string(),
          ]
      );
  }

  #[test]
  fn attached_store() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("a-flag")
                  .with_flag_char('a')
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .with_store(StoreType::Value, StoreSyntax::Attached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Attached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("boolean")
                  .with_flag_char('b')
                  .with_about("Boolean", "Testing bool")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--a-flag=AttachedValue".to_string(),
              "-p=8080".to_string(),
              "--port=0420".to_string(),
              "-p=8081".to_string(),
              "-b".to_string(),
              "-bp=1234".to_string(),
              "-a".to_string(),
              "--port=5678".to_string(),
              "-bap=9510".to_string(),
          ]);
      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("a-flag"));
      assert_eq!(
          cli.get_flag_store("a-flag").unwrap().as_value().unwrap(),
          &vec!["AttachedValue".to_string()]
      );
      assert!(cli.is_flag_entered("boolean"));
      assert!(cli.get_flag_store("boolean").unwrap().exists());
      assert!(cli.is_flag_entered("port"));
      assert_eq!(
          cli.get_flag_store("port").unwrap().as_value().unwrap(),
          &vec![
              "8080".to_string(),
              "0420".to_string(),
              "8081".to_string(),
              "1234".to_string(),
              "5678".to_string(),
              "9510".to_string(),
          ]
      );
  }
  ```

- [x] **Step 2: Run store tests to verify they pass**
  Run: `cargo test --test parser_tests store`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/parser_tests/store.rs
  git commit -m "refactor(tests): migrate store tests"
  ```

---

### Task 5: Migrate Regression Tests

**Files:**
- Modify: `tests/parser_tests/regression.rs`

- [x] **Step 1: Move regression tests into `tests/parser_tests/regression.rs`**
  Add the migrated tests:
  ```rust
  use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

  #[test]
  fn regression_detached_store() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("port-other")
                  .with_flag_char('o')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("boolean")
                  .with_flag_char('b')
                  .with_about("Boolean", "Testing bool")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--port-other".to_string(),
              "8080".to_string(),
              "--port".to_string(),
              "0123".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("port"));
      assert_eq!(
          cli.get_flag_store("port").unwrap().as_value().unwrap(),
          &vec!["0123".to_string()]
      );
      assert!(cli.is_flag_entered("port-other"));
      assert_eq!(
          cli.get_flag_store("port-other")
              .unwrap()
              .as_value()
              .unwrap(),
          &vec!["8080".to_string()]
      );
  }

  #[test]
  fn regression_attached_store() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Attached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("port-other")
                  .with_flag_char('o')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Attached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("boolean")
                  .with_flag_char('b')
                  .with_about("Boolean", "Testing bool")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--port-other=8080".to_string(),
              "--port=0123".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("port"));
      assert_eq!(
          cli.get_flag_store("port").unwrap().as_value().unwrap(),
          &vec!["0123".to_string()]
      );
      assert!(cli.is_flag_entered("port-other"));
      assert_eq!(
          cli.get_flag_store("port-other")
              .unwrap()
              .as_value()
              .unwrap(),
          &vec!["8080".to_string()]
      );
  }

  #[test]
  fn regression_single_short_flag_overwrite() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("boolean")
                  .with_flag_char('b')
                  .with_about("Boolean", "Testing bool")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "-p".to_string(),
              "8080".to_string(),
              "-p".to_string(),
              "0123".to_string(),
              "-bp".to_string(),
              "4567".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("port"));
      assert_eq!(
          cli.get_flag_store("port").unwrap().as_value().unwrap(),
          &vec!["8080".to_string(), "0123".to_string(), "4567".to_string()]
      );
  }

  #[test]
  fn regression_optional_store_no_value() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("a-flag")
                  .with_flag_char('a')
                  .with_short_about("Testing text")
                  .with_long_about("Long testing text \n lorem ipsum")
                  .with_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "An optional port")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("boolean")
                  .with_flag_char('b')
                  .with_about("Boolean", "Testing bool")
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "-a".to_string(),
              "detachedValue".to_string(),
              "-p".to_string(),
              "8080".to_string(),
              "--port".to_string(),
              "0420".to_string(),
              "-p".to_string(),
              "8081".to_string(),
              "-bp".to_string(),
              "1234".to_string(),
              "-a".to_string(),
              "--port".to_string(),
              "5678".to_string(),
              "-bap".to_string(),
              "9510".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("a-flag"));
      assert_eq!(
          cli.get_flag_store("a-flag").unwrap().as_value().unwrap(),
          &vec!["detachedValue".to_string()]
      );
      assert!(cli.is_flag_entered("port"));
      assert_eq!(
          cli.get_flag_store("port").unwrap().as_value().unwrap(),
          &vec![
              "8080".to_string(),
              "0420".to_string(),
              "8081".to_string(),
              "1234".to_string(),
              "5678".to_string(),
              "9510".to_string(),
          ]
      );
  }
  ```

- [x] **Step 2: Run regression tests to verify they pass**
  Run: `cargo test --test parser_tests regression`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/parser_tests/regression.rs
  git commit -m "refactor(tests): migrate regression parser tests"
  ```

---

### Task 6: Migrate Error Parser Tests

**Files:**
- Modify: `tests/parser_tests/errors.rs`

- [x] **Step 1: Move error tests into `tests/parser_tests/errors.rs`**
  Add the migrated tests:
  ```rust
  use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

  #[test]
  fn test_missing_argument_long_flag() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("port")
                  .with_about("Port", "Port flag")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec!["test".to_string(), "--port".to_string()]);
      assert!(cli.is_err());
      let err = cli.err().unwrap();
      assert_eq!(err.source_name(), "eshu::parser");
      let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
      match leaf_err {
          eshu::EshuErrorKind::MissingArgument {
              flag,
              expected_syntax,
          } => {
              assert_eq!(flag, "--port");
              assert_eq!(expected_syntax, "--port VALUE");
          }
          _ => panic!("Expected EshuErrorKind::MissingArgument"),
      }
  }

  #[test]
  fn test_missing_argument_short_flag() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("port")
                  .with_flag_char('p')
                  .with_about("Port", "Port flag")
                  .with_required_store(StoreType::Value, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec!["test".to_string(), "-p".to_string()]);
      assert!(cli.is_err());
      let err = cli.err().unwrap();
      assert_eq!(err.source_name(), "eshu::parser");
      let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
      match leaf_err {
          eshu::EshuErrorKind::MissingArgument {
              flag,
              expected_syntax,
          } => {
              assert_eq!(flag, "-p (--port)");
              assert_eq!(expected_syntax, "-p VALUE");
          }
          _ => panic!("Expected EshuErrorKind::MissingArgument"),
      }
  }
  ```

- [x] **Step 2: Run error tests to verify they pass**
  Run: `cargo test --test parser_tests errors`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/parser_tests/errors.rs
  git commit -m "refactor(tests): migrate error parser tests"
  ```

---

### Task 7: Implement Key-Value Flag Parsing Tests

**Files:**
- Modify: `tests/parser_tests/key_value.rs`

- [x] **Step 1: Write key-value parsing tests**
  Add the following new test cases to `tests/parser_tests/key_value.rs`:
  ```rust
  use eshu::{Cli, CliFlag, StoreSyntax, StoreType, EshuErrorKind};

  #[test]
  fn test_key_value_detached() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("config")
                  .with_flag_char('c')
                  .with_about("config", "Config flags")
                  .with_required_store(StoreType::KeyValue, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--config".to_string(),
              "host=localhost".to_string(),
              "-c".to_string(),
              "port=8080".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_flag_entered("config"));
      let store = cli.get_flag_store("config").unwrap().as_key_value().unwrap();
      assert_eq!(store.get("host").unwrap(), "localhost");
      assert_eq!(store.get("port").unwrap(), "8080");
  }

  #[test]
  fn test_key_value_attached() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("config")
                  .with_flag_char('c')
                  .with_about("config", "Config flags")
                  .with_required_store(StoreType::KeyValue, StoreSyntax::Attached)
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--config=host=localhost".to_string(),
              "-c=port=8080".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      let store = cli.get_flag_store("config").unwrap().as_key_value().unwrap();
      assert_eq!(store.get("host").unwrap(), "localhost");
      assert_eq!(store.get("port").unwrap(), "8080");
  }

  #[test]
  fn test_key_value_malformed() {
      let cli = Cli::new("test-cli")
          .add_flag(
              CliFlag::new("config")
                  .with_about("config", "Config flags")
                  .with_required_store(StoreType::KeyValue, StoreSyntax::Detached)
                  .build()
                  .unwrap(),
          )
          .with_version("0.0.0")
          .try_parse_custom(vec![
              "test".to_string(),
              "--config".to_string(),
              "malformed_no_equals".to_string(),
          ]);

      assert!(cli.is_err());
      let err = cli.unwrap_err();
      let leaf_err = err.downcast_ref::<EshuErrorKind>().unwrap();
      match leaf_err {
          EshuErrorKind::MissingArgument { flag, expected_syntax } => {
              assert_eq!(flag, "--config");
              assert_eq!(expected_syntax, "--config KEY=VALUE");
          }
          _ => panic!("Expected MissingArgument for malformed key-value pair"),
      }
  }
  ```

- [x] **Step 2: Run key-value tests to verify they pass**
  Run: `cargo test --test parser_tests key_value`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/parser_tests/key_value.rs
  git commit -m "test(parser): add key-value flag parsing edge cases"
  ```

---

### Task 8: Implement XffValue Casting Tests

**Files:**
- Modify: `tests/parser_tests/xff.rs`

- [x] **Step 1: Write tests for Store to XffValue casting**
  Add the following tests to `tests/parser_tests/xff.rs`:
  ```rust
  use std::collections::BTreeMap;
  use eshu::Store;
  use athena::XffValue;

  #[test]
  fn test_xff_exists() {
      let store = Store::Exists;
      let xff: XffValue = store.into();
      assert_eq!(xff, XffValue::from(true));
  }

  #[test]
  fn test_xff_value() {
      let store = Store::Value(vec!["hello".to_string(), "world".to_string()]);
      let xff: XffValue = store.into();
      let arr = xff.as_array().unwrap();
      assert_eq!(arr.len(), 2);
      assert_eq!(arr[0], XffValue::from("hello"));
      assert_eq!(arr[1], XffValue::from("world"));
  }

  #[test]
  fn test_xff_key_value() {
      let mut map = BTreeMap::new();
      map.insert("a".to_string(), "1".to_string());
      map.insert("b".to_string(), "2".to_string());
      let store = Store::KeyValue(map);
      let xff: XffValue = store.into();
      let obj = xff.as_object().unwrap();
      assert_eq!(obj.get("a").unwrap(), &XffValue::from("1"));
      assert_eq!(obj.get("b").unwrap(), &XffValue::from("2"));
  }
  ```

- [x] **Step 2: Run XffValue tests to verify they pass**
  Run: `cargo test --test parser_tests xff`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/parser_tests/xff.rs
  git commit -m "test(utils): add Store to XffValue casting tests"
  ```

---

### Task 9: Expand Subcommand Tests

**Files:**
- Modify: `tests/subcmd_tests.rs`

- [x] **Step 1: Write expanded subcommand tests**
  Add the new tests to the end of `tests/subcmd_tests.rs`:
  ```rust
  #[test]
  fn nested_subcommand() {
      let cli = Cli::new("test-cli")
          .with_version("0.0.0")
          .add_command(std::rc::Rc::new(
              CliCmd::new("parent")
                  .with_about("parent command", "parent command long")
                  .add_command(std::rc::Rc::new(
                      CliCmd::new("child")
                          .with_about("child command", "child command long")
                          .add_flag(
                              CliFlag::new("flag")
                                  .with_about("flag", "flag")
                                  .build()
                                  .unwrap(),
                          )
                          .build()
                          .unwrap(),
                  ))
                  .build()
                  .unwrap(),
          ))
          .try_parse_custom(vec![
              "test-cli".to_string(),
              "parent".to_string(),
              "child".to_string(),
              "--flag".to_string(),
          ]);

      assert!(cli.is_ok());
      let cli = cli.unwrap();
      assert!(cli.is_subcommand_entered("parent"));
      
      let parent_cli = cli.get_subcommand_cli("parent").unwrap();
      assert!(parent_cli.is_subcommand_entered("child"));
      
      let child_cli = parent_cli.get_subcommand_cli("child").unwrap();
      assert!(child_cli.is_flag_entered("flag"));
      
      // Verify isolation: parent should not have parsed the child flag
      assert!(!parent_cli.is_flag_entered("flag"));
  }

  #[test]
  fn unrecognized_subcommand() {
      let cli = Cli::new("test-cli")
          .with_version("0.0.0")
          .add_command(std::rc::Rc::new(
              CliCmd::new("cmd")
                  .with_about("cmd", "cmd")
                  .build()
                  .unwrap(),
          ))
          .try_parse_custom(vec![
              "test-cli".to_string(),
              "unknown".to_string(),
          ]);

      assert!(cli.is_err());
      let err = cli.unwrap_err();
      let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
      match leaf_err {
          eshu::EshuErrorKind::UnrecognizedCommand(cmd) => {
              assert_eq!(cmd, "unknown");
          }
          _ => panic!("Expected EshuErrorKind::UnrecognizedCommand"),
      }
  }
  ```

- [x] **Step 2: Run subcommand tests**
  Run: `cargo test --test subcmd_tests`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/subcmd_tests.rs
  git commit -m "test(subcmd): add nested subcommand and error edge cases"
  ```

---

### Task 10: Expand Builder Validation Tests

**Files:**
- Modify: `tests/builder_tests.rs`

- [x] **Step 1: Write expanded builder validation tests**
  Add the new tests to the end of `tests/builder_tests.rs`:
  ```rust
  #[test]
  fn duplicate_flag_name_error() {
      let cli = Cli::new("test-cli")
          .with_version("0.0.0")
          .add_flag(
              CliFlag::new("flag")
                  .with_about("f", "f")
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("flag")
                  .with_about("duplicate", "duplicate")
                  .build()
                  .unwrap(),
          )
          .try_parse_custom(vec!["test-cli".to_string()]);

      assert!(cli.is_err());
      let err = cli.unwrap_err();
      let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
      match leaf_err {
          eshu::EshuErrorKind::Duplicate(msg) => {
              assert!(msg.contains("Flag"));
          }
          _ => panic!("Expected EshuErrorKind::Duplicate"),
      }
  }

  #[test]
  fn duplicate_flag_char_error() {
      let cli = Cli::new("test-cli")
          .with_version("0.0.0")
          .add_flag(
              CliFlag::new("flag1")
                  .with_flag_char('f')
                  .with_about("f", "f")
                  .build()
                  .unwrap(),
          )
          .add_flag(
              CliFlag::new("flag2")
                  .with_flag_char('f')
                  .with_about("duplicate char", "duplicate char")
                  .build()
                  .unwrap(),
          )
          .try_parse_custom(vec!["test-cli".to_string()]);

      assert!(cli.is_err());
      let err = cli.unwrap_err();
      let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
      match leaf_err {
          eshu::EshuErrorKind::Duplicate(msg) => {
              assert!(msg.contains("Short Flag Char"));
          }
          _ => panic!("Expected EshuErrorKind::Duplicate"),
      }
  }

  #[test]
  fn invalid_flag_name_whitespace() {
      let flag = CliFlag::new("invalid name")
          .with_about("f", "f")
          .build();
      assert!(flag.is_err());
      let err = flag.unwrap_err();
      let leaf_err = err.downcast_ref::<eshu::EshuErrorKind>().unwrap();
      match leaf_err {
          eshu::EshuErrorKind::InvalidFlagName(msg) => {
              assert!(msg.contains("whitespace"));
          }
          _ => panic!("Expected EshuErrorKind::InvalidFlagName"),
      }
  }
  ```

- [x] **Step 2: Run builder tests**
  Run: `cargo test --test builder_tests`
  Expected: PASS

- [x] **Step 3: Commit**
  Run:
  ```bash
  git add tests/builder_tests.rs
  git commit -m "test(builder): add flag validation edge cases"
  ```
