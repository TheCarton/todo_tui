use crossterm::event::KeyCode;

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

pub fn keycode_to_action(key: KeyCode) -> Option<ActionKind> {
    match key {
        ADD_TASK_KEYCODE => Some(ADD_TASK_ACTION),
        EDIT_MODE_KEYCODE => Some(EDIT_MODE_ACTION),
        SHUFFLE_TASK_KEYCODE => Some(SHUFFLE_TASK_ACTION),
        QUIT_KEYCODE => Some(QUIT_ACTION),
        MARK_TASK_DONE_KEYCODE => Some(MARK_TASK_DONE_ACTION),
        MARK_TASK_IN_PROGRESS_KEYCODE => Some(MARK_TASK_IN_PROGRESS_ACTION),
        KEYS_HINT_KEYCODE => Some(KEYS_HINT_ACTION),
        FOCUS_TITLE_KEYCODE => Some(FOCUS_TITLE_ACTION),
        FOCUS_DESCRIPTION_KEYCODE => Some(FOCUS_DESCRIPTION_ACTION),
        CHANGE_MODE_KEYCODE => Some(CHANGE_MODE_ACTION),
        INCREMENT_DUE_DATE_BY_1_KEYCODE => Some(INCREMENT_DUE_DATE_BY_1_ACTION),
        DECREMENT_DUE_DATE_BY_1_KEYCODE => Some(DECREMENT_DUE_DATE_BY_1_ACTION),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub(crate) struct InputKey {
    pub(crate) key_code: KeyCode,
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
}

const ADD_TASK_KEYCODE: KeyCode = KeyCode::Char('a');
const ADD_TASK_ACTION: ActionKind = ActionKind::AddTask(InputKey {
    key_code: ADD_TASK_KEYCODE,
    name: "a",
    description: "add mode",
});

const EDIT_MODE_KEYCODE: KeyCode = KeyCode::Char('e');
const EDIT_MODE_ACTION: ActionKind = ActionKind::EditMode(InputKey {
    key_code: EDIT_MODE_KEYCODE,
    name: "e",
    description: "edit mode",
});

const SHUFFLE_TASK_KEYCODE: KeyCode = KeyCode::Char('r');
const SHUFFLE_TASK_ACTION: ActionKind = ActionKind::ShuffleTasks(InputKey {
    key_code: SHUFFLE_TASK_KEYCODE,
    name: "r",
    description: "shuffle tasks",
});

const QUIT_KEYCODE: KeyCode = KeyCode::Char('q');
const QUIT_ACTION: ActionKind = ActionKind::Quit(InputKey {
    key_code: QUIT_KEYCODE,
    name: "q",
    description: "quit",
});

const MARK_TASK_DONE_KEYCODE: KeyCode = KeyCode::Char('d');
const MARK_TASK_DONE_ACTION: ActionKind = ActionKind::MarkTaskDone(InputKey {
    key_code: MARK_TASK_DONE_KEYCODE,
    name: "d",
    description: "mark task done",
});

const MARK_TASK_IN_PROGRESS_KEYCODE: KeyCode = KeyCode::Char('D');
const MARK_TASK_IN_PROGRESS_ACTION: ActionKind = ActionKind::MarkTaskInProgress(InputKey {
    key_code: MARK_TASK_IN_PROGRESS_KEYCODE,
    name: "D",
    description: "mark task in progress",
});

const KEYS_HINT_KEYCODE: KeyCode = KeyCode::Char('?');
const KEYS_HINT_ACTION: ActionKind = ActionKind::KeysHint(InputKey {
    key_code: KEYS_HINT_KEYCODE,
    name: "?",
    description: "mark task in progress",
});

const FOCUS_TITLE_KEYCODE: KeyCode = KeyCode::Char('t');
const FOCUS_TITLE_ACTION: ActionKind = ActionKind::FocusTitle(InputKey {
    key_code: FOCUS_TITLE_KEYCODE,
    name: "a",
    description: "focus title input field",
});

const FOCUS_DESCRIPTION_KEYCODE: KeyCode = KeyCode::Char('T');
const FOCUS_DESCRIPTION_ACTION: ActionKind = ActionKind::FocusDescription(InputKey {
    key_code: FOCUS_DESCRIPTION_KEYCODE,
    name: "d",
    description: "focus title description field",
});

const CHANGE_MODE_KEYCODE: KeyCode = KeyCode::Esc;
const CHANGE_MODE_ACTION: ActionKind = ActionKind::ChangeMode(InputKey {
    key_code: CHANGE_MODE_KEYCODE,
    name: "Escape",
    description: "change modes",
});

const INCREMENT_DUE_DATE_BY_1_KEYCODE: KeyCode = KeyCode::Char('y');
const INCREMENT_DUE_DATE_BY_1_ACTION: ActionKind = ActionKind::IncrementDueDate(InputKey {
    key_code: INCREMENT_DUE_DATE_BY_1_KEYCODE,
    name: "y",
    description: "increment due date",
});

const DECREMENT_DUE_DATE_BY_1_KEYCODE: KeyCode = KeyCode::Char('Y');
const DECREMENT_DUE_DATE_BY_1_ACTION: ActionKind = ActionKind::DecrementDueDate(InputKey {
    key_code: DECREMENT_DUE_DATE_BY_1_KEYCODE,
    name: "Y",
    description: "decrement due date by 1",
});

pub const DELETE_CHAR_KEYCODE: KeyCode = KeyCode::Backspace;
