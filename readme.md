# iced.rs

[https://github.com/RGGH/iced_tutorial](https://github.com/RGGH/iced_tutorial)`<br>`

### [YouTube iced GUI video](https://youtu.be/X9Hebeionj8)

[screenshot](resources/screenshot.png)

## Examples

[https://github.com/iced-rs/awesome-iced?tab=readme-ov-file](https://github.com/iced-rs/awesome-iced?tab=readme-ov-file)

[https://github.com/RGGH/iced_tutorial/blob/main/tutorial/first_app.md](https://github.com/RGGH/iced_tutorial/blob/main/tutorial/first_app.md)

## Getting Started

An Iced program is built with several key parts that work together to define the application's functionality and appearance. Here's a breakdown of the terms:

**Imports:**

- Similar to other programming languages, `imports` bring functionality from external libraries like Iced itself or other helper libraries into your program.

**fn main:**

- This is the main function where your program execution starts. In Iced, it typically sets up the initial state of your application and launches the UI loop.

**struct:**

- Structures define custom data types that hold related pieces of information. In Iced, you'll often create structs to represent the state of your UI elements like buttons or text fields.

**theme**:

```rust
fn theme(&self) -> iced::Theme {
    iced::Theme::Dracula
}
```

**impl:**

- The `impl` keyword is used to define methods that act on your structs. These methods can handle user interactions, update the state, or perform calculations related to your UI elements.
- eg

```rust
impl Sandbox for GroceryList {
```

**update:**

- The `update` function is called whenever the user interacts with your UI (like clicking a button or typing in a field). It receives the current application state and the event that triggered the update. Here, you update the state based on the user interaction.

```rust
    fn update(&mut self, message: Self::Message) {
        match message {
            // Update logic
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // View logic
    }
```

**view:**

- The `view` function takes the current application state and translates it into the visual representation of your UI. This function defines how different UI elements are arranged and displayed on the screen.

**title:**

- The `title` is a string that sets the title of your application window.
  ---

### These parts work together in a loop:

- **Initialization:** The `fn main` sets up the initial state and launches the UI loop.
- **Event Handling:** When a user interacts with the UI, an event is generated.
- **Update:** The `update` function receives the event and updates the application state accordingly.

```rust
    fn update(&mut self, **message**: Self::Message) {
```

- **View:** The `view` function takes the updated state and generates the new UI visuals.

  (contains rows and columns)
- **Render:** The updated UI is rendered on the screen.
- **Repeat:** This loop continues, constantly updating the UI based on user interactions and state changes.

Iced provides a clean way to separate the logic of your application (`update`) from the visual representation (`view`) making your code more maintainable and easier to understand.

## How to run in main (Sandbox version)

```rust
struct **GroceryList** {
	grocery_items: Vec<String>,
	input_value: String,
}

.
.
.

fn main() -> iced::Result {
    **GroceryList**::run(Settings::default())
}
```

If you are a just getting started with the library, this trait offers a simpler interface.

Unlike an application, Sandbox cannot run any asynchronous actions or be initialized with some external flags. However, both traits are very similar and upgrading from a sandbox is very straightforward.

Therefore, it is recommended to always start by implementing this trait and upgrade only once necessary.
