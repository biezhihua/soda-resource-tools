#!/bin/bash

cross build --target x86_64-apple-darwin --release

cross build --target x86_64-pc-windows-gnu --release

cross build --target x86_64-unknown-linux-gnu --release