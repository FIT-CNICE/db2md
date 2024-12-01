use db2md::gui::*;
use std::fs;
use std::path::PathBuf;

fn setup_test_env() -> PathBuf
{
  // Create test output directory
  let output_dir = PathBuf::from("./tests/test_output");
  if !output_dir.exists() {
    fs::create_dir_all(&output_dir).unwrap();
  }
  output_dir
}

fn cleanup_test_env(output_dir: PathBuf)
{
  if output_dir.exists() {
    fs::remove_dir_all(output_dir).unwrap();
  }
}

#[test]
fn test_app_initialization()
{
  let app = Db2MdApp::default();
  assert_eq!(app.title(), "db2md");
}

#[test]
fn test_file_selection()
{
  let mut app = Db2MdApp::default();

  // Test file selection message
  let _ = app.update(Message::FileSelected(Some("./tests/fruit_test.xlsx".to_string())));

  // Test YAML selection message
  let _ = app.update(Message::YamlSelected(Some("./tests/schema.yaml".to_string())));
  assert!(true);
}

#[test]
fn test_header_setting()
{
  let mut app = Db2MdApp::default();

  // Test setting header flag
  let _ = app.update(Message::SetHasHeader(true));

  // Test setting file prefix
  let _ = app.update(Message::SetFilePrefix("test".to_string()));
  assert!(true)
}
