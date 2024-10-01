use crossterm::event::KeyCode;

macro_rules! const_action_kinds {
    (
        $(
            ($keycode_name:ident, $const_name:ident) { $variant:ident, $key_code:expr, $name:expr, $description:expr }
        ),*
    ) => {
        $(
            // Define a constant for the KeyCode
            pub(crate) const $keycode_name: KeyCode = $key_code;


            // Define a constant for each ActionKind variant with InputKey
            pub(crate) const $const_name: ActionKind = ActionKind::$variant(InputKey {
                key_code: $key_code,
                name: $name,
                description: $description,
            });
        )*

        // Define a constant array containing all ActionKind variants
        pub(crate) const ACTION_KINDS: [ActionKind; count!($($const_name),*)] = [
            $(
                $const_name
            ),*
        ];

      // Define a function that maps KeyCode to ActionKind
        pub(crate) fn keycode_to_actionkind(keycode: KeyCode) -> Option<ActionKind> {
            match keycode {
                $(
                    $keycode_name => Some($const_name),
                )*
                _ => None, // Default case when keycode does not match
            }
        }
    }
}

// Macro to count the number of constants being defined
macro_rules! count {
    ($($element:expr),*) => {
        <[()]>::len(&[$(count_expr!($element)),*])
    };
}

macro_rules! count_expr {
    ($element:expr) => {
        ()
    };
}

const_action_kinds! {
    (ADD_TASK_KEYCODE, ADD_TASK) { AddTask, KeyCode::Char('a'), "Add Task", "Add a new task" },
    (EDIT_MODE_KEYCODE, EDIT_MODE){ EditMode, KeyCode::Char('e'), "Edit Mode", "Enter edit mode" },
    (CHOOSE_TASK_KEYCODE , CHOOSE_TASK ){ ShuffleTasks, KeyCode::Char('r'), "shuffle tasks", "choose a task"},
    (QUIT_KEYCODE , QUIT ){ Quit, KeyCode::Char('q'), "quit", "quit app"},
    (MARK_COMPLETE_KEYCODE , MARK_COMPLETE ){MarkTaskDone, KeyCode::Char('d'), "mark task done", "mark task as done."},
    (MARK_INCOMPLETE, MARK_INCOMPLETE_KEYCODE ) { MarkTaskInProgress, KeyCode::Char('D'), "mark task not done", "blabla"},
    (KEYS_HINT,  KEYS_HINT_KEYCODE ){ KeysHint, KeyCode::Char('?'), "keys help", "get help"},
    (FOCUS_TITLE,  FOCUS_TITLE_KEYCODE ){ FocusTitle, KeyCode::Char('t'), "focus title field", "for typing idk"},
    (FOCUS_DESCRIPTION,  FOCUS_DESCRIPTION_KEYCODE ){ FocusDescription, KeyCode::Char('T'), "focus description", "desc" },
    (CHANGE_MODE,  CHANGE_MODE_KEYCODE ){ ChangeMode, KeyCode::Esc, "change modes", "escape" },
    (INCREMENT_DATE,  INCREMENT_DATE_KEYCODE ){ IncrementDueDate, KeyCode::Char('y'), "increment date", "change due date"},
    (DECREMENT_DATE,  DECREMENT_DATE_KEYCODE ){ DecrementDueDate, KeyCode::Char('Y'), "decrement date", "change due date"}
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub(crate) enum ActionKind {
    AddTask(InputKey),
    EditMode(InputKey),
    ShuffleTasks(InputKey),
    Quit(InputKey),
    MarkTaskDone(InputKey),
    MarkTaskInProgress(InputKey),
    KeysHint(InputKey),
    FocusTitle(InputKey),
    FocusDescription(InputKey),
    ChangeMode(InputKey),
    IncrementDueDate(InputKey),
    DecrementDueDate(InputKey),
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub(crate) struct InputKey {
    pub(crate) key_code: KeyCode,
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
}

pub const DELETE_CHAR_KEYCODE: KeyCode = KeyCode::Backspace;
