use crate::task::Task;

#[test]
fn serialize_deserialize_task() {
    let task = Task::new("Test", "Test Serialize");
    
    // Serialice
    let ser = serde_json::to_string_pretty(&task).unwrap();

    // Desserialize
    let des = serde_json::from_str(ser.as_str()).unwrap_or(Task::new("Error".to_string(), "".to_string()));

    assert_eq!(task, des)
}