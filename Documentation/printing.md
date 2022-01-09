# Printing

Often - printing to the screen is necessary, which is why the functions below allow you to do that. The following array of functions
is available at your disposal.

- `info!` - Prints an informative message to the screen
- `success!` - Something good happened
- `error!` - An error occurred
- `warn!` - Warn the user
- `emergency!` - The user must act as soon as possible
- `kprint!` - Prints black and white text to the screen *without adding a newline*
- `kprintln!` - Prints black and white text to the screen *with a newline*

## Debug-Printing
Often you just want to dump some values. That can be done using the `debug!` macro. It will print debug text, but only if the project is compiled in debug mode (Change in `Esque.toml`)