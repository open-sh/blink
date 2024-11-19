# Crate Architecture

```plaintext
blink/
├── Cargo.toml
├── crates/
│   ├── app/              # Main entrypoint
│   ├── state/            # State management
│   ├── tui/              # Terminal interface rendering
│   ├── cli/              # CLI interface
│   ├── protocol_manager/ # Manage protocols
│   ├── protocols/        # Native protocol implementations
│   │   ├── http/
│   │   ├── graphql/
│   │   ├── grpc/
│   │   ├── tcp/
│   │   ├── udp/
│   │   └── ...
│   ├── storage/          # Store data and requests
│   ├── docs_generator/   # Generate documentation
│   ├── config/           # Config parsing and manipulation
│   ├── plugins/          # Plugin system
│   └── utils/            # Shared utilities
├── plugins/
│   └── custom_protocol/  # Example plugin for custom protocol
├── README.md
└── blink.toml            # Local project config
```

# Data Flow and State Manipulation

## 1. Application Initialization

**`app/`**:

- **Load Configurations**: Starts the application by loading configurations via **`config/`**. This includes global settings and user preferences.
- **Initialize Modules**: Initializes necessary modules such as **`tui/`**, **`protocol_manager/`**, **`storage/`**, etc.
- **Set Up Global State**: Establishes the initial global state of the application, which includes the current mode, loaded requests, and any other necessary state data.

## 2. User Interaction

### TUI Mode

- **User Interface**: The user interacts with the terminal interface, navigating through requests, viewing responses, etc.
- **Event Capturing**: **`tui/`** captures user events (key presses, mouse clicks, window resizing, etc.).
- **Vim Mode Logic**: **`tui/`** incorporates Vim mode logic, interpreting events according to the current mode (Normal, Insert, Visual). Note: Modes should be restricted to windows, for instace, some components of the UI will not be editable.
- **Command Generation**: Based on the interpretation, **`tui/`** generates commands to be executed.
- **Command Dispatch**: Resulting commands are sent to **`app/`** for processing.

### CLI Mode

- **Command Line Input**: The user provides commands and arguments via the command line.
- **Input Processing**: **`cli/`** processes the input, parses arguments, and validates options.
- **Direct Interaction**: **`cli/`** interacts directly with **`app/`** to execute commands without the need for a TUI.

## 3. Command Processing in `tui/`

### Vim Mode Integration

- **Mode Management**: **`tui/`** maintains Vim mode states, switching between modes based on user input (e.g., pressing `i` to enter Insert mode).
- **Event Interpretation**: Input events are interpreted within the context of the current mode.
  - **Normal Mode**: Navigation and command execution.
  - **Insert Mode**: Text input and editing.
  - **Visual Mode**: Selection and manipulation of multiple items.
- **Examples**:
  - **Normal Mode**: Pressing `j` moves the cursor down.
  - **Insert Mode**: Typing characters inserts them into a text field.

### Sending Commands to `app/`

- **Command Creation**: Interpreted inputs are converted into command objects or messages.
- **State Changes**: While **`tui/`** may handle some immediate interface updates, any changes to the global application state are sent to **`app/`**.
- **Asynchronous Handling**: Commands may be queued or handled asynchronously to ensure responsive UI.

## 4. State Management in `app/`

### Command Processing

- **Receive Commands**: **`app/`** receives commands from **`tui/`** or **`cli/`**.
- **State Update**: Updates the global state of the application based on the command.
  - **Examples**:
    - Updating cursor position in a list of requests.
    - Modifying the details of a request (e.g., changing headers, body).
    - Changing application settings (e.g., toggling dark mode).

### Interaction with Other Modules

- **`protocol_manager/`**:
  - **Request Handling**: To send requests using specific protocols.
  - **Protocol Selection**: Determines which protocol to use based on request configuration.
- **`storage/`**:
  - **Data Persistence**: Saves requests, responses, and other data to disk.
  - **Data Retrieval**: Loads saved data when needed.
- **`docs_generator/`**:
  - **Documentation Generation**: Triggers the generation of API documentation based on stored requests and responses.

## 5. Interface Update

**`tui/`**:

- **State Notification**: **`app/`** notifies **`tui/`** about changes in the application state that affect the interface.
- **Rendering Updates**: **`tui/`** uses **`ratatui`** to render updates in the terminal, ensuring the interface reflects the current state.
  - **Examples**:
    - Updating the list of requests after adding a new one.
    - Displaying the response from a sent request.
    - Showing error messages or notifications.

## 6. Data Flow Example - TUI Mode

**Input Event**:

- The user presses the `k` key in the terminal to navigate up.

**Interpretation in `tui/`**:

- **Vim Normal Mode**: **`tui/`** recognizes that it's in Normal mode.
- **Command Interpretation**: Interprets `k` as "move cursor up".
- **Command Generation**: Creates a command object `MoveCursor(Direction::Up)`.

**Sending to `app/`**:

- **Command Dispatch**: The command is sent to **`app/`** for execution.

**Processing in `app/`**:

- **State Update**: **`app/`** updates the cursor position in the global state.
- **Additional Actions**: Determines if additional actions are needed, such as loading more data if at the top of the list.

**Interface Update**:

- **State Notification**: **`app/`** notifies **`tui/`** of the state change.
- **Rendering**: **`tui/`** re-renders the interface to reflect the new cursor position.

## 7. Data Flow Example - Sending a Request

**User Command**:

- The user selects a request and issues a command to send it, either via TUI (e.g., pressing `Enter`) or CLI (e.g., `blink send request_id`).

**Processing in `app/`**:

- **Command Reception**: **`app/`** receives the `SendRequest(RequestID)` command.
- **Request Retrieval**: Fetches the request details from **`storage/`** or the current state.

**Interaction with `protocol_manager/`**:

- **Protocol Determination**: **`protocol_manager/`** identifies the appropriate protocol to use (e.g., HTTP, gRPC).
- **Request Execution**: Sends the request using the selected protocol.
- **Response Handling**: Receives the response or error.

**Update in `app/`**:

- **State Update**: **`app/`** updates the state with the response data.
- **Data Persistence**: Stores the response in **`storage/`** if necessary.
- **Error Handling**: Updates state with error information if the request failed.

**Interface Update**:

- **State Notification**: **`app/`** notifies **`tui/`** to update the interface.
- **Rendering**: **`tui/`** displays the response data or error messages to the user.

---

# Additional Details

## Global State Management

- **State Structure**: The global state managed by **`app/`** might include:

  - Current mode (e.g., Normal, Insert).
  - Cursor positions in various lists or views.
  - Loaded requests and responses.
  - User configurations and preferences.

- **State Synchronization**: Ensures consistency between the state and the interface, especially when changes occur due to asynchronous operations like network requests.

## Asynchronous Operations

- **Concurrency Handling**: Network requests and other I/O operations may be asynchronous.
- **Event Loop**: The application may use an event loop or async runtime to manage concurrency.
- **Feedback to User**: Provides progress indicators or loading states in the interface.

## Error Handling

- **Graceful Degradation**: The application should handle errors gracefully, providing meaningful feedback to the user.
- **Logging**: Errors and important events can be logged via **`utils/`** for debugging purposes.

## Extensibility via Plugins

- **Plugin System**: **`plugins/`** allows third-party developers to extend the application's functionality.
- **Custom Protocols**: New protocols can be added via plugins, expanding the application's capabilities.
- **APIs for Plugins**: Well-defined interfaces and APIs facilitate safe interaction between plugins and the core application.

## Configuration Management

- **Global Configurations**: Stored in a common location (e.g., `~/.config/blink/config.toml`).
- **Project-specific Configurations**: Local configurations in `blink.toml` override or extend global settings.
- **Dynamic Reloading**: The application may support reloading configurations without restarting.

## Documentation Generation

- **Formats Supported**: The **`docs_generator/`** may support generating documentation in various formats like HTML, PDF, Markdown.
- **Customization**: Users can configure the style and content of the generated documentation.
- **Automation**: Documentation generation can be triggered manually or automated as part of workflows.
