#!/bin/bash

cargo clippy --no-deps --locked -- -Dwarnings -Dclippy::pedantic
