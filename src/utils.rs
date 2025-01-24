use std::fs;

use chrono::{Locale, NaiveDateTime};

use crate::task::Task;

/// Lee el archivo "tasks.json" y obtiene las tareas alamacenadas en él.
/// Si el archivo no existe, lo crea y retorna un vector vacío.
pub fn read_tasks() -> Vec<Task> {
    let tasks = fs::read_to_string("tasks.json")
        .unwrap_or_else(|_| {
            let empty: Vec<Task> = Vec::new();
            fs::write("tasks.json", serde_json::to_string(&empty).unwrap()).unwrap();
            serde_json::to_string(&empty).unwrap()
        });

    serde_json::from_str(&tasks).unwrap()
}

/// Sobreescribe el archivo "tasks.json" con el vector de tareas pasado como parámetro
pub fn write_tasks(tasks: Vec<&Task>) {
    fs::write("tasks.json", serde_json::to_string(&tasks).unwrap()).unwrap();
}

/// Convierte el tipo [NaiveDateTime] en [String] con el formato `%A %d de %B del %Y - %r`
pub fn format_date_time(date_time: NaiveDateTime) -> String {
    let date = date_time.date();
    let time = date_time.time();
    let date_localized = date.format_localized("%A %d de %B del %Y", Locale::es_PE).to_string();
    let time_formated = time.format("%r").to_string();

    return format!("{} - {}", date_localized, time_formated);
}