use std::fmt::Display;

use chrono::{Local, NaiveDateTime};
use iced::{widget::{button, column, combo_box, container, horizontal_space, row, scrollable, text, text_editor, text_input}, Background, Element, Length, Theme};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::format_date_time;

/// Representa un tarea almacenada.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Task {
    id: Uuid,
    pub title: String,
    pub description: String,
    pub status: Status,
    created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}
impl Task {
    /// Crea una nueva intancia de [Task] a partir de un titulo y una descripción.
    pub fn new<T: AsRef<str>>(title: T, description: T) -> Self {
        let now = Local::now().naive_local();
        let title = title.as_ref().to_string();
        let description = description.as_ref().to_string();

        Task {
            id: Uuid::new_v4(),
            title, 
            description,
            status: Status::ToDo,
            created_at: now,
            modified_at: now,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn modified(&mut self) {
        self.modified_at = Local::now().naive_local()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn modified_at(&self) -> NaiveDateTime {
        self.modified_at
    }

    /// Edita esta instancia de [Task] 
    pub fn modify(&mut self, title: Option<String>, description: Option<String>, status: Option<Status>)  {
        if let Some(title) = title {
            self.title = title
        }

        if let Some(description) = description {
            self.description = description
        }

        if let Some(status) = status {
            self.status = status
        }

        self.modified_at = Local::now().naive_local();
    }
}

/// Representa el estado de una instacia de [Task]
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Done,
    InProgress,
    ToDo
}

/// Represeta el estado local de una instancia de [Task].
/// * _`state:`_ Indíca si la tarea está en vista estática o en edición
/// * _`fields:`_ Ayuda a manejar la lógica de estado y pintado de la instancia 
#[derive(Debug)]
pub struct TaskView {
    task: Task,
    state: State,

    fields: Field
}

#[derive(Debug)]
pub struct Field {
    title: String,
    status: Status,
    combo_state: combo_box::State<Status>,
    text_editor_content: text_editor::Content
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum State {
    #[default]
    Static,
    Edit
}

#[derive(Debug, Clone)]
pub enum Message {
    /// Modificar la instancia actual de [Task]
    Modify {
        title: Option<String>,
        description: Option<String>,
        status: Option<Status>
    },

    // Manejo de estado y pintado
    SetTitle(String),
    SetDescription(text_editor::Action),
    SetStatus(Status),

    /// Intercambia de vista estática a edición
    ToggleState,
    /// Se ha actualizado la instancia de [Task]
    Update,

    /// Notificar que se ha eliminado una instancia de [Task]
    Delete(Uuid),
}
impl Status {
    pub const ALL: &'static [Self] = &[Status::Done, Status::InProgress, Status::ToDo];
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            Status::Done => "Terminada",
            Status::InProgress => "En progreso",
            Status::ToDo => "Pendiente"
        })
    }
}

impl TaskView {
    /// Obtiene una referencia a la instancia de [Task] que pinta la istancia actual de [TaskView]
    pub fn get_task(&self) -> &Task {
        &self.task
    }

    /// Obtiene una referencia mutable a la intancia de [Task] que pinsta la instancia actual de [TaskView]
    pub fn get_task_mut(&mut self) -> &mut Task {
        &mut self.task
    }

    /// Lógica de actualización de estado
    pub fn update(&mut self,  message: Message) -> iced::Task<Message> {
        match message {
            // Modificar esta tarea
            Message::Modify { title, description, status } => {
                self.task.modify(title, description, status);

                return iced::Task::done(Message::ToggleState).chain(iced::Task::done(Message::Update))
            },

            // Actualización deestado
            Message::SetTitle(title) => self.fields.title = title,
            Message::SetDescription(action) => self.fields.text_editor_content.perform(action),
            Message::SetStatus(status) => self.fields.status = status,
            Message::ToggleState => match self.state {
                State::Edit => self.state = State::Static,
                State::Static => self.state = State::Edit
            },

            _ => ()
            // Estos mensajes son para el estado global
            // Message::Delete(id)
            // Message::Update
        }

        iced::Task::none()
    }

    /// Lógica de pintado
    pub fn view(&self) -> iced::Element<Message> {
        container(match self.state {
            State::Static => self.static_view(),
            State::Edit => self.edit_view()
        })
        .style(container::rounded_box)
        .height(Length::Shrink)
        .max_height(300)
        .into()
    }

    /// Vista estática
    fn static_view(&self) -> Element<Message> {
        column![].push(
            // Titulo
            row![].push(text(&self.task.title))
            .push(horizontal_space())

            // Estatus
            .push(container(text(self.task.status.to_string())).style(|theme: &Theme| {
                let extended_palette = theme.extended_palette();
                let style = container::rounded_box(theme);

                style.background(match self.task.status {
                    Status::Done => Background::Color(extended_palette.success.strong.color),
                    Status::ToDo => Background::Color(extended_palette.danger.strong.color),
                    Status::InProgress => Background::Color(extended_palette.secondary.weak.color)
                })
            }).padding(5))
        )
        // Descripción
        .push(
            container(
                scrollable(
                    text(&self.task.description)
                    .width(Length::Fill)
                ).height(Length::Shrink)
            ).max_height(75)
        )
        .push(
            {
                let column = column![]
                // Creación
                .push(text!("Creado: {}", format_date_time(self.task.created_at)).style(text::secondary));
                
                // Edición
                if self.task.created_at != self.task.modified_at {
                    column.push(
                        text!("Última modificación: {}", format_date_time(self.task.modified_at)).style(text::secondary)
                    )
                }else {
                    column
                }        
            }
        )
        // Botones de acción
        .push(row![]
            // Editar
            .push(button("Editar").on_press(Message::ToggleState))
            // Eliminar
            .push(button("Eliminar").on_press(Message::Delete(self.task.id)))
            .push(horizontal_space())
            .spacing(10)
        )
        .padding(10)
        .spacing(10)
        .into()
    }

    // Vista de edición
    fn edit_view(&self) -> Element<Message> {
        column![]
        .push(row![]
            // Titulo
            .push(
                text_input("Título...", &self.fields.title)
                .on_input(Message::SetTitle)
                .width(Length::FillPortion(2))
            )
            .push(horizontal_space())
            // Status
            .push(combo_box(
                &self.fields.combo_state, 
                "Estado...", 
                Some(&self.fields.status), 
                Message::SetStatus
            ).width(Length::Fixed(100.0)))
        )
        .push(
            // Descripción
            container(
                text_editor(&self.fields.text_editor_content)
                .placeholder("Descripción...")
                .on_action(Message::SetDescription)
                .height(Length::Fill)
            ).height(75)
        )
        // Botones de acción
        .push(row![].push(
            // Aceptar edición
            button("Aceptar").on_press_with(||{
                let title = if !self.fields.title.is_empty() && self.fields.title != self.task.title {
                    Some(self.fields.title.clone())
                }else {None};

                let description = if self.fields.text_editor_content.text().trim() != self.task.description {
                    Some(self.fields.text_editor_content.text().trim().to_string())
                }else {None};
                
                let status = if self.fields.status != self.task.status {
                    Some(self.fields.status)
                }else {None};

                Message::Modify { title, description, status }
            }))
            // Cancelar edición
            .push(button("Cancelar").on_press(Message::ToggleState))
            .push(horizontal_space())
            .spacing(10)
        )
        .padding(10)
        .spacing(10)
        .into()
    }
}

impl From<&Task> for TaskView {
    fn from(task: &Task) -> Self {
        TaskView { 
            state: State::Static, 
            fields: Field { 
                title: task.title.clone(), 
                status: task.status.clone(), 
                combo_state: combo_box::State::new(Status::ALL.to_vec()), 
                text_editor_content: text_editor::Content::with_text(&(task.description.clone()))
            },
            task: task.to_owned()
        }
    }
}

impl From<Task> for TaskView {
    fn from(value: Task) -> Self {
        TaskView::from(&value)
    }
}

impl From<&TaskView> for Task {
    fn from(task_view: &TaskView) -> Self {
        task_view.task.to_owned()
    }
}