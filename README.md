# PraccJS — Practice your JS 

PraccJS is a desktop REPL client for organizing and executing JS code snippets. It's built using Tauri, Rust, Svelte, Oxc and Rustyscript.

https://github.com/user-attachments/assets/19625151-d3fd-466c-9946-7e085914a18e

## Notes

It not support async js, typescript at the moment. Feel free to create issues and share your thoughts.

## Prerequisites
- Intall Rust. Verify installation with `rustc --version` or `cargo -V`.
- Install Node.js 18+

## Installing Dependencies

```zsh
npm install  # Install npm dependencies
cd src-tauri  # Navigate to the Rust folder
cargo install  # Install cargo dependencies
```

## Starting the Development Server
```zsh
npm run tauri dev
```

## Building the Project
```zsh
npm run tauri build
```

After building, you can find the distributables in `./src-tauri/target/release/bundle/`