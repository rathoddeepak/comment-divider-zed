# Comment Divider for Zed

A Zed Code Editor extension that helps you organize your code by creating styled comment separators. This extension is inspired by and aims to replicate the functionality of the popular VS Code Comment Divider extension.

## Features

- **Main Header**: Creates a prominent 3-line block comment header
- **Subheader**: Creates a single-line comment with centered text
- **Solid Line**: Inserts a simple comment line divider
- **Multi-language Support**: Automatically uses appropriate comment syntax for different programming languages
- **Customizable**: Configurable length, alignment, fill characters, and text transforms

## Default Styles

### Main Header (`Shift+Alt+X`)
```javascript
/* -------------------------------------------------------------------------- */
/*                                Example text                                */
/* -------------------------------------------------------------------------- */
```

### Subheader (`Alt+X`)
```javascript
/* ------------------------------ Example text ------------------------------ */
```

### Solid Line (`Alt+Y`)
```javascript
/* -------------------------------------------------------------------------- */
```

## Language Support

The extension automatically detects your file's language and uses the appropriate comment syntax:

- **JavaScript/TypeScript**: `//` comments
- **Python**: `#` comments
- **HTML**: `<!-- -->` comments
- **CSS/C**: `/* */` comments
- **Rust/C++/Java/Go**: `//` comments
- **Ruby/Shell/Bash**: `#` comments
- **SQL/Lua**: `--` comments
- **And many more...**

## Installation

### Development Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/comment-divider-zed.git
   cd comment-divider-zed
   ```

2. Install Rust (required for Zed extensions):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. In Zed, open the Extensions panel and click "Install Dev Extension"
4. Select the directory containing your extension

### Publishing (Future)

This extension will be published to the Zed Extensions Registry. Once published, you can install it directly from Zed's Extensions panel.

## Usage

### Keyboard Shortcuts

- `Shift+Alt+X`: Create main header from current line text
- `Alt+X`: Create subheader from current line text
- `Alt+Y`: Insert solid line divider

### Slash Commands

You can also use slash commands in Zed's Agent Panel:

- `/divider [text]`: Create a subheader with optional text
- `/header [text]`: Create a main header with text

### Workflow

1. Type your header text on a line
2. Use one of the keyboard shortcuts
3. The extension will replace the line with a styled comment divider

Or for empty dividers:
1. Position cursor where you want the divider
2. Press `Alt+Y` for a solid line

## Configuration

Add these settings to your Zed settings file (`~/.config/zed/settings.json`):

```json
{
  "comment_divider": {
    "length": 80,
    "should_length_include_indent": false,
    "main_header_filler": "-",
    "main_header_height": "block",
    "main_header_align": "center",
    "main_header_transform": "none",
    "subheader_filler": "-",
    "subheader_align": "center",
    "subheader_transform": "none",
    "line_filler": "-",
    "languages_map": {
      "custom_language": ["//"],
      "toml": ["#", "#"],
      "scss": ["//"]
    }
  }
}
```

### Settings Explained

- **length**: Total character width of dividers (default: 80)
- **should_length_include_indent**: Whether to account for indentation in length calculation
- **main_header_filler**: Character used to fill main header lines
- **main_header_height**: "block" for 3-line headers, "line" for single line
- **main_header_align**: Text alignment - "left", "center", or "right"
- **main_header_transform**: Text transform - "none", "uppercase", "lowercase", "capitalize"
- **subheader_***: Same options as main header but for subheaders
- **line_filler**: Character used for solid line dividers
- **languages_map**: Custom comment characters for specific languages

### Custom Language Support

To add support for a new language or override existing ones:

```json
{
  "comment_divider": {
    "languages_map": {
      "mylang": ["//"],
      "otherlang": ["/*", "*/"],
      "python": ["#", "#"]
    }
  }
}
```

The array can have 1 element (for line comments) or 2 elements (for block comments with start/end).

## Examples

### With Indentation
```javascript
function myFunction() {
    /* -------------------------------- Variables ------------------------------- */
    let x = 1;

    /* --------------------------------- Logic --------------------------------- */
    return x * 2;
}
```

### Different Languages
```python
# -------------------------------- Python Example ------------------------------ #

def hello_world():
    pass
```

```html
<!-- ---------------------------- HTML Example ----------------------------- -->

<div>Content here</div>
```

```rust
// -------------------------------- Rust Example ------------------------------- //

fn main() {
    println!("Hello, world!");
}
```

## Development

### Building

```bash
cargo build --target wasm32-wasi
```

### Project Structure

```
comment-divider/
├── extension.toml          # Extension metadata and commands
├── Cargo.toml             # Rust dependencies
├── src/
│   └── lib.rs            # Main extension logic
├── settings/
│   └── comment-divider.json  # Settings schema
└── README.md
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with different languages and settings
5. Submit a pull request

## Troubleshooting

### Extension Not Loading
- Ensure Rust is installed via rustup (not homebrew)
- Check that you have the latest version of Zed
- Try restarting Zed after installation

### Language Not Supported
- Add custom language mapping in settings
- Check that the language identifier matches Zed's language modes
- File an issue for built-in support

### Keyboard Shortcuts Not Working
- Check for conflicts with other extensions
- Verify you're in an editor context (not terminal/panel)
- Try using the Command Palette instead

## Roadmap

- [ ] Interactive configuration UI
- [ ] More text transform options
- [ ] Custom templates/styles
- [ ] Integration with Zed's snippet system
- [ ] Batch operations for multiple lines

## License

MIT License - see LICENSE file for details.

## Acknowledgments

Inspired by the original [Comment Divider](https://marketplace.visualstudio.com/items?itemName=stackbreak.comment-divider) extension for VS Code by stackbreak.
