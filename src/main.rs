pub mod utils;
pub mod task_tracker;
pub mod task;

#[cfg(test)]
mod tests;

use iced::{application, keyboard::{self, key::Named, Key}, widget::{button, column, container, focus_next, focus_previous, horizontal_space, row, scrollable, text, text_editor::Binding, text_editor, text_input}, window::Settings, Background, Element, Length, Size, Subscription, Theme};
use task::Status;
use task_tracker::{Message, TaskTracker};
use utils::write_tasks;

fn main () -> iced::Result {
    application("Task Tracker", TaskTracker::update, TaskTracker::view)
    .window(Settings{
        position: iced::window::Position::Centered,
        min_size: Some(Size::new(450.0, 580.0)),
        size: Size::new(450.0, 580.0),
        ..Default::default()
    })
    .subscription(TaskTracker::subscriptions)
    .run()
}

impl TaskTracker {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::FocusNext => return focus_next(),
            Message::FocusPrev => return focus_previous(),
            
            Message::Delete(id) => self.remove_task(id),
            Message::SetDescription(action) => self.description.perform(action),
            Message::SetTitle(title) => self.title = title,
            Message::SetQueryText(text) => self.filter.text = text,
            Message::SetQueryStatus(status)  => self.filter.status = status,
            Message::Create(title, description) => {
                if title.trim().is_empty() || description.trim().is_empty() {
                    return iced::Task::none();
                }

                self.add_task(title, description);
                self.title.clear();
                self.description = text_editor::Content::new();
            }

            Message::TaskMessage(id, task_message) => match task_message {
                task::Message::Delete(id) => self.remove_task(id),
                task::Message::Update => write_tasks(self.get_tasks()),
                _ => {
                    let task_view = self.tasks.iter_mut().find(|tv| tv.get_task().id() == id);

                    if let Some(task_view) = task_view {
                        return task_view.update(task_message).map(move |m|Message::TaskMessage(id, m))
                    }
                }
            }
        }

        iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![]
        .push(text("Lista de Tareas").size(32))
        .push(
            text_input("Título...", &self.title).on_input(Message::SetTitle)
            .on_submit(Message::FocusNext)
        )
        .push(text_editor(&self.description)
            .placeholder("Descripción...")
            .on_action(Message::SetDescription)
            .key_binding(|key_press|{
                if key_press.key == Key::Named(Named::Enter) && key_press.modifiers.shift() {
                    return Some(Binding::Custom(Message::Create(self.title.clone(), self.description.text().trim().to_string())))
                }
                Binding::from_key_press(key_press)
            })
            
        )
        .push(row![]
            .push(
                button("Crear Tarea")
                .on_press_with(|| Message::Create(self.title.clone(), self.description.text().trim().to_string()))
            )
            .push(horizontal_space())
        )
        
        .push(text("Buscar"))
        .push(
            text_input("Buscar por titulo o descripción...", &self.filter.text)
            .on_input(Message::SetQueryText)
        )
        .push(container(
                row![]
                .push(
                    button("Todas").on_press(Message::SetQueryStatus(None))
                    .style(if self.filter.status.is_none() {
                        button::primary
                    }else {button::secondary})
                )
                .push(
                    button("Pendientes").on_press(Message::SetQueryStatus(Some(task::Status::ToDo)))
                    .style(match self.filter.status {
                        Some(status) if status == Status::ToDo => button::primary,
                        _ => button::secondary
                    })
                )
                .push(
                    button("En progreso").on_press(Message::SetQueryStatus(Some(task::Status::InProgress)))
                    .style(match self.filter.status {
                        Some(status) if status == Status::InProgress => button::primary,
                        _ => button::secondary
                    })
                )
                .push(
                    button("Terminadas").on_press(Message::SetQueryStatus(Some(task::Status::Done)))
                    .style(match self.filter.status {
                        Some(status) if status == Status::Done => button::primary,
                        _ => button::secondary
                    })
                )
                .spacing(5)
            ).style(|theme: &Theme| {
                container::background(
                    Background::Color(theme.extended_palette().background.strong.color)
                )
            })
            .padding(5)
            .width(Length::Fill)
        )
        .push(
            container(
                scrollable(
                    column![]
                    .extend(self.filtered_tasks())
                    .spacing(5)
                ).spacing(5)
            ).height(Length::Fill)
        )
        .padding(15)
        .spacing(5)
        .into()   
    }

    fn subscriptions(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| {
            if key == Key::Named(Named::Tab) {
                if modifiers.shift() {
                    return Some(Message::FocusPrev)
                }else {
                    return Some(Message::FocusNext)
                }
            }

            None
        })    
    }

    fn filtered_tasks(&self) -> Vec<iced::Element<Message>> {
        let query = &self.filter.text;
        match self.filter.status {
            Some(status) => self.by_status(status)
            .filter(|tv|{
                let task = tv.get_task();

                return task.title.contains(query) || task.description.contains(query)
            })
            .map(|task|task.view().map(|m| Message::TaskMessage(task.get_task().id(), m))).collect(),
            None => self.tasks.iter()
                .filter(|tv| {
                    let task = tv.get_task();

                    return task.title.contains(query) || task.description.contains(query)
                })
                .map(|task| task.view().map(|m|Message::TaskMessage(task.get_task().id(), m))).collect(),
        }

    }
}