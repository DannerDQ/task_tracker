use chrono::NaiveDateTime;
use iced::widget::text_editor;
use uuid::Uuid;

use crate::task::{self, Status, Task, TaskView};
use crate::utils::{read_tasks, write_tasks};

#[derive(Debug)]
pub struct TaskTracker {
    pub tasks: Vec<TaskView>,
    
    pub title: String,
    pub description: text_editor::Content,

    pub filter: Query
}
impl Default for TaskTracker {
    fn default() -> Self {
        TaskTracker {
            tasks: read_tasks().iter().map(|task| TaskView::from(task)).collect(),
            title: String::new(),
            description: text_editor::Content::new(),
            
            filter: Query { text: String::new(), status: None }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub text: String,
    pub status: Option<Status>
}

#[derive(Debug, Clone)]
pub enum Message {
    Delete(Uuid),
    SetTitle(String),
    SetDescription(text_editor::Action),

    SetQueryText(String),
    SetQueryStatus(Option<Status>),

    Create(String, String),

    TaskMessage(Uuid, task::Message),

    FocusNext,
    FocusPrev
}

impl TaskTracker {
    pub fn add_task(&mut self, title: String, description: String) {
        self.tasks.push(TaskView::from(Task::new(title, description)));
        
        write_tasks(self.get_tasks());
    }

    pub fn remove_task(&mut self, id: Uuid) {
        self.tasks.retain(|tv| tv.get_task().id() != id);
        write_tasks(self.get_tasks());
    }

    pub fn update_task(&mut self, id: Uuid, title: Option<String>, description: Option<String>, status: Option<Status>) {
        let task = self.get_task_mut(id);

        if let Some(task) = task {
            if let Some(title) = title.clone() {
                task.set_title(title);
            }
            if let Some(description) = description.clone() {
                task.set_description(description);
            }
            if let Some(status) = status.clone() {
                task.set_status(status);
            }

            task.modified();
            write_tasks(self.get_tasks());
        }
    }

    pub fn get_task(&self, id: Uuid) -> Option<&Task> {
        self.get_tasks_iter().find(|task| task.id() == id)
    }

    pub fn get_task_mut(&mut self, id:Uuid) -> Option<&mut Task> {
        self.get_tasks_iter_mut().find(|task| task.id() == id)
    }

    pub fn get_tasks(&self) -> Vec<&Task> {
        self.get_tasks_iter().collect()
    }

    pub fn get_tasks_iter(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter().map(|tv|tv.get_task())
    }

    pub fn get_tasks_iter_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        self.tasks.iter_mut().map(|tv| tv.get_task_mut())
    }

    pub fn by_status(&self, status: Status) -> impl Iterator<Item = &TaskView> {
        self.tasks.iter().filter(move |task_view| task_view.get_task().status == status)
    }

    pub fn get_tasks_by_date(&self, date: NaiveDateTime) -> Vec<&Task> {
        self.get_tasks_iter().filter(|task| task.created_at() == date).collect()
    }

    pub fn by_title_or_description(&self, query: &str) -> Vec<&Task> {
        self.get_tasks_iter().filter(|task| task.title.contains(query) || task.description.contains(query)).collect()
    }

    pub fn get_tasks_by_date_range(&self, start: NaiveDateTime, end: NaiveDateTime) -> Vec<&Task> {
        self.get_tasks_iter().filter(|task| task.created_at() >= start && task.created_at() <= end).collect()
    }
}

