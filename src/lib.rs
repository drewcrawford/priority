use std::fmt::Display;

/**
Defines an abstract priority for tasks.
*/
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    /**
    The task runs at UI priority.

    Generally, we expect this task to paint the screen or respond to user input as soon as possible.

    In systems with a single-threaded UI, the task runs on the UI thread.
    Work performed at this priority may block the UI thread.
    */
    UserInteractive,
    /**
    The task runs at a high priority.

    Use this priority for tasks that respond to user input, and expect to complete quickly, before
    the user switches focus (e.g. on the order of a second).

    Use this priority for tasks where the user is likely waiting on the input.
    */
    UserInitiated,
    /**
    This task runs at a medium priority.

    Use this priority for tasks with expected such that the user may switch focus before they complete (e.g. on the order of a 10+ seconds).

    Use this priority for tasks where we might display a progress bar and ideally the user can move onto other work during completion.
    */
    Utility,

    /**
    This task runs at a low priority.

    Use this priority for tasks that are not time-sensitive, and can run in the background.

    Use this priority for tasks that are not visible to the user, and do not require user input.
    */
    Background
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::UserInteractive => write!(f, "UserInteractive"),
            Priority::UserInitiated => write!(f, "UserInitiated"),
            Priority::Utility => write!(f, "Utility"),
            Priority::Background => write!(f, "Background"),
        }
    }
}