# Land2Port Project Instructions

## Code Style
- Use Rust (Edition 2024) for all development
- Follow standard Rust formatting conventions (rustfmt)
- Use `snake_case` for variables, functions, and modules
- Use `PascalCase` for types, traits, and enums
- Use `SCREAMING_SNAKE_CASE` for constants

## Error Handling
- Use `anyhow::Result<T>` for error propagation
- Use `anyhow::Context` for adding context to errors
- Use `map_err` with descriptive error messages
- Prefer `?` operator for error propagation

## Async Programming
- Use `#[tokio::main]` for main function
- Use `async fn` for I/O operations
- Use `.await?` for async error handling
- Prefer async/await over manual Future handling

## Architecture
- Use trait-based design for polymorphic behavior
- Implement `Default` trait for configuration structs
- Use composition over inheritance
- Keep functions focused and single-purpose

## Video Processing
- Use `usls` library for YOLO model integration
- Support multiple model versions and scales
- Implement sophisticated crop logic based on object count
- Use percentage-based similarity thresholds for smoothing

## CLI and Configuration
- Use `argh` derive macros for argument parsing
- Provide sensible defaults for all options
- Use environment variables for sensitive configuration
- Never hardcode API keys in source code

## Testing
- Use `#[cfg(test)]` modules for unit tests
- Test both success and failure cases
- Use descriptive test names
- Test edge cases and boundary conditions

## Documentation
- Use `///` for public API documentation
- Include examples in documentation when helpful
- Document all public functions, structs, and traits
- Use `# Arguments`, `# Returns`, and `# Examples` sections
