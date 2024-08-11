use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

static TEST_JSON_FILE: &str = "test_db.json";


fn create_todo_list_command() -> (Command, tempfile::TempDir) {
    let temp_dir = tempdir().expect("Failed to create a temporary directory");
    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");

    cmd.env("APP_ENV", "test")
       .env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE));
    (cmd, temp_dir)
}

#[test]
fn test_add_task() {
    let (mut cmd, _temp_dir) = create_todo_list_command();

    cmd.arg("add")
        .arg("Task")
        .arg("Description")
        .arg("1-1-2021 12:00")
        .arg("Category")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task added successfully!"));
}

#[test]
fn test_add_and_select_task() {
    let (mut cmd, temp_dir) = create_todo_list_command();

    cmd.arg("add")
        .arg("Task")
        .arg("Description")
        .arg("1-1-2021 12:00")
        .arg("Category")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("select")
        .arg("*")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task")
            .and(predicate::str::contains("Description"))
            .and(predicate::str::contains("Category")));
}

#[test]
fn test_add_and_select_all_tasks() {
    let (mut cmd, temp_dir) = create_todo_list_command();

    cmd.arg("add")
        .arg("Task A1")
        .arg("Description A1")
        .arg("1-1-2021 12:00")
        .arg("Category A1")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");

    cmd.arg("add")
        .arg("Task B1")
        .arg("Description B1")
        .arg("1-1-2021 12:00")
        .arg("Category B1")
        .assert()
        .success();


    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("select")
        .arg("*")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task A")
            .and(predicate::str::contains("Description A"))
            .and(predicate::str::contains("Category A")));
}

#[test]
fn test_add_and_done_task() {
    let (mut cmd, temp_dir) = create_todo_list_command();

    cmd.arg("add")
        .arg("Test_done_task")
        .arg("Test Description")
        .arg("1-1-2021 12:00")
        .arg("Test Category")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("done")
        .arg("Test_done_task")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task marked as done!"));

    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("select")
        .assert()
        .success()
        .stdout(predicate::str::contains("status: true"));
}

#[test]
fn test_add_and_delete_task() {
    let (mut cmd, temp_dir) = create_todo_list_command();

    cmd.arg("add")
        .arg("Test Task")
        .arg("test_add_and_delete_task Description")
        .arg("1-1-2021 12:00")
        .arg("Test Category")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("delete")
        .arg("Test Task")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task deleted successfully!"));

    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join("tasks.json"))
        .arg("select")
        .arg("*")
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks match the given criteria."));
}
#[test]
fn test_add_and_update_task() {
    let (mut cmd, temp_dir) = create_todo_list_command();

    // Add a task
    cmd.arg("add")
        .arg("TestTask")
        .arg("Test Description")
        .arg("1-1-2021 12:00")
        .arg("Test Category")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task added successfully!"));

    // Verify the task was added
    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("select")
        .arg("*")
        .assert()
        .success()
        .stdout(predicate::str::contains("TestTask")
            .and(predicate::str::contains("Test Description"))
            .and(predicate::str::contains("Test Category")));

    // Update the task
    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("update")
        .arg("TestTask")
        .write_stdin("UpdatedTask\nUpdated Description\n1-1-2022 12:00\nUpdated Category\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task updated successfully!"));

    // Verify the task was updated
    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("select")
        .arg("*")
        .assert()
        .success()
        .stdout(predicate::str::contains("UpdatedTask")
            .and(predicate::str::contains("Updated Description"))
            .and(predicate::str::contains("2022-01-01T12:00:00Z"))
            .and(predicate::str::contains("Updated Category")));
}

#[test]
fn test_add_multiple_tasks_and_select() {
    let (mut cmd, temp_dir) = create_todo_list_command();

    // Add multiple tasks
    cmd.arg("add")
        .arg("Task 1")
        .arg("Description 1")
        .arg("1-1-2021 12:00")
        .arg("Category A")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task added successfully!"));


    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");

    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("add")
        .arg("Task 2")
        .arg("Description 2")
        .arg("1-1-2021 12:00")
        .arg("Category B")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task added successfully!"));

    // Check if all tasks are present
    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");

    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("select")
        .arg("*")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task 1")
            .and(predicate::str::contains("Description 1"))
            .and(predicate::str::contains("Category A"))
            .and(predicate::str::contains("Task 2"))
            .and(predicate::str::contains("Description 2"))
            .and(predicate::str::contains("Category B")));

    // Select tasks by category
    let mut cmd = Command::cargo_bin("todolist").expect("Failed to find the 'todolist' binary");
    cmd.env("TODO_FILE", temp_dir.path().join(TEST_JSON_FILE))
        .arg("select")
        .arg("category = \"Category A\"")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task 1")
            .and(predicate::str::contains("Description 1"))
            .and(predicate::str::contains("Category A"))
            .and(predicate::str::contains("Task 2").not()));
}
