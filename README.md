# KAE: Terminal Taskmaster 🦀

A To-Do application with a sleek `ratatui`-based Terminal User Interface (TUI) and Command Line Interface (CLI). Manage your tasks without leaving the terminal!

## ✨ Features

*   **Dual Interface**:
    *   **CLI**: Quickly add, list, update, and initialize tasks directly from your command line.
    *   **Interactive TUI**: A rich, `ratatui`-powered terminal interface for a more visual and interactive experience.
*   **Task Management**:
    *   Create new tasks with names and descriptions.
    *   List all tasks or filter by ID/name (CLI).
    *   Update task details (name, description, status) via CLI or directly within the TUI.
    *   Cycle through task statuses: `Todo` -> `InProgress` -> `Done`.
*   **Persistent Storage**: Tasks are saved locally in a simple `todo/.todo.json` file.
*   **Built with Rust**: Ensuring speed, safety, and efficiency.

## 🚀 Installation & Setup

1.  **Prerequisites**:
    *   Ensure you have Rust and Cargo installed. If not, follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

2.  **Clone the Repository**:
    ```bash
    # If you have a git repo setup, otherwise skip this
    # git clone <your-repository-url>
    # cd kae
    ```

3.  **Build the Project**:
    ```bash
    cargo build
    ```
    For a release build (optimized):
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/debug/kae` or `target/release/kae`.

4.  **Initialize Project Storage (First time)**:
    Before adding tasks, you need to initialize the storage:
    ```bash
    cargo run -- init
    # Or, if you built for release:
    # ./target/release/kae init
    ```
    This will create a `.todo/todo.json` file in the directory where you run the command.

## ⌨️ Usage

### Command Line Interface (CLI)

The general command structure is `cargo run -- <COMMAND> [OPTIONS]` or `./target/release/kae <COMMAND> [OPTIONS]`.

*   **Initialize a new project store**:
    ```bash
    cargo run -- init
    ```

*   **Add a new task**:
    ```bash
    cargo run -- add --name "My New Task" --description "Detailed description of the task."
    ```

*   **List tasks (opens TUI)**:
    ```bash
    cargo run -- list
    ```
    *   Filter by name (in CLI, TUI will show all):
        ```bash
        cargo run -- list --name "Specific Task"
        ```
    *   Fetch by ID (in CLI, TUI will show all):
        ```bash
        cargo run -- list --id "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
        ```
    *   List all (explicitly, though `list` defaults to this for TUI):
        ```bash
        cargo run -- list --all
        ```

*   **Update an existing task**:
    ```bash
    cargo run -- update --id "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" --name "Updated Task Name" --description "New details" --status "InProgress"
    ```
    *   You can update any combination of name, description, or status. Status options are: `Todo`, `InProgress`, `Done` (case-insensitive).

### Terminal User Interface (TUI)

Launch the TUI by running the `list` command:

```bash
cargo run -- list
```

**Keybindings within the TUI**:

*   **Navigation**:
    *   `↓` / `Arrow Down`: Move selection down.
    *   `↑` / `Arrow Up`: Move selection up.
    *   `Home`: Select the first task.
    *   `End`: Select the last task.
    *   `←` / `Arrow Left`: Unselect the current task.
*   **Actions**:
    *   `Tab`: Toggle status of the selected task (Todo -> InProgress -> Done -> Todo). Changes are saved automatically.
    *   `e`: Edit the **name** of the selected task.
        *   Type to change, `Enter` to save, `Esc` to cancel.
    *   `d`: Edit the **description** of the selected task.
        *   Type to change, `Enter` to save, `Esc` to cancel.
*   **General**:
    *   `q`: Quit the application.
    *   `Esc`: Exit edit mode and return to View mode.

## 🔧 Development & Internals

*   **UI**: Built with [`ratatui`](https://ratatui.rs).
*   **CLI Parsing**: Uses [`clap`](https://crates.io/crates/clap).
*   **Serialization**: Handled by [`serde`](https://crates.io/crates/serde) for JSON.
*   **UUIDs**: Generated using the [`uuid`](https://crates.io/crates/uuid) crate.
*   **Logging**: Employs [`tracing`](https://crates.io/crates/tracing).

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/mundhrakeshav/kae/issues).