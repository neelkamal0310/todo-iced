use iced::{
    pure::{
        widget::{Button, Checkbox, Column, Container, Row, Text, TextInput},
        Sandbox,
    },
    Error, Settings,
};

#[derive(Clone)]
struct Task {
    task_name: String,
    finished: bool,
}

#[derive(Clone)]
struct Todo {
    task: String,
    finished: bool,
    task_list: Vec<Task>,
}

#[derive(Debug, Clone)]
enum Action {
    CheckboxToggled(bool),
    TaskChanged(String),
    CreateTodoEntry,
}

impl Sandbox for Todo {
    type Message = Action;

    fn new() -> Self {
        Todo {
            task: String::from(""),
            finished: false,
            task_list: vec![],
        }
    }

    fn title(&self) -> String {
        String::from("Simple Todo")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Action::CheckboxToggled(is_checked) => {
                self.finished = is_checked;
            }
            Action::TaskChanged(task) => {
                self.task = task;
            }
            Action::CreateTodoEntry => {
                self.task_list.push(Task {
                    task_name: self.task.clone(),
                    finished: self.finished,
                });
            }
        }
    }

    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        let mut column = Column::new()
            .push(Row::new().push(Text::new("Task: ")).push(TextInput::new(
                "Enter task here",
                &self.task,
                Action::TaskChanged,
            )))
            .push(Checkbox::new(
                self.finished,
                "Is finished: ",
                Action::CheckboxToggled,
            ))
            .push(Button::new("Submit").on_press(Action::CreateTodoEntry));
        for task in &self.task_list {
            column = column.push(Text::new(&task.task_name));
        }
        Container::new(column).center_x().center_y().into()
    }
}

fn main() -> Result<(), Error> {
    Todo::run(Settings::default())
}
