# IO Recorder

IO Recorder allows to save a sequence of IO's done via `embedded-hal` and save them to a file for inspection.


## Features

- [x] Record all actions sent to a [`digital::OutputPin`].
- [ ] Wrap an existing [`digital::OutputPin`] and record all values sent to it to allow recording while driving a real IO.
- [ ] Apply recorded values to an [`digital::OutputPin`] to allow replay a sequence.
- [ ] Provide recorded data as a [`digital::InputPin`].
- [x] Save recorded data to

## Future Goals

- [ ] Support more `embedded-hal` types than just GPIO.

[`digital::OutputPin`]: https://docs.rs/embedded-hal/latest/embedded_hal/digital/trait.OutputPin.html
[`digital::InputPin`]: https://docs.rs/embedded-hal/latest/embedded_hal/digital/trait.InputPin.html
