# Introducing ToDoTui

ToDoTui is a work-in-progress to-do app that runs in the terminal and isn’t just a list. Users can add tasks blazingly fast, and without removing
 their hands from the keyboard. Tasks can be organized into projects, and tasks are displayed based on a simple round-robin algorithm.

## Philosophy of Use  
ToDoTui is meant to take tasks off your mind when they’re distracting you as quickly as possible, and not make matters worse by distracting you further.
Later, when you’re remembering that there was something you wanted to work on, you can quickly shuffle through your tasks and find it.

ToDoTui wants you to make a decision about a task. It tracks how many times you skip a task, and allows you to find these often-skipped tasks and abandon them.

There’s nothing wrong with abandoning tasks. ToDoTui is meant to help you focus on what you think is important, not pretend you’ll be able to do
every single thing that comes to your mind.

ToDoTui wants to help you celebrate progress by tracking tasks you’ve completed.

## Workflow: Adding a Task

The user opens ToDoTui with a new task they want ToDoTui to store. They type a simple description of a task,
and optionally give it a due date and assign it to a project. This is meant to have very few distractions,
and so does not show a list of unfinished tasks to the user while they’re trying to quickly jot down a task.

## Workflow: Finding a Task

The user opens ToDoTui, either with a project they intend to work on or not. If they have a project in mind,
they select that project, and cycle through tasks until they see one they decide to work on. If they have no task in mind,
the default project of ToDoTui cycles through all other projects.

## Workflow: Cleaning up Tasks

By default on startup, ToDoTui shows the last displayed task. A displayed task can be marked as finished, abandoned, assigned to a project,
or simply shuffled behind the other tasks. A user can also open the Very Old Tasks wizard, which displays tasks that have been skipped
many times and are long past their due dates. This encourages users to abandon tasks that, realistically, they are never going to do.
