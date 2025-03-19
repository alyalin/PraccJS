<div align="center">
   <img align="center" width="128px" src="https://github.com/user-attachments/assets/faeb29db-da58-4425-b0cb-58593b55fddc" />
	<h1 align="center"><b>PraccJS</b></h1>
	<p align="center">
		Desktop REPL client for organizing and executing JS code snippets. It's built using Tauri, Rust, Svelte, Oxc and Rustyscript.
    <br />
    <br />
    <br />
    <b>Download for </b>
    macOS (<a href="https://github.com/alyalin/PraccJS/releases/download/0.2.0/PraccJS_0.2.0_aarch64.dmg">Apple Silicon</a> |
      <a href="https://github.com/alyalin/PraccJS/releases/download/0.2.0/PraccJS_0.2.0_x64.dmg">Intel</a>) ·
		Linux (<a href="https://github.com/alyalin/PraccJS/releases/download/0.2.0/PraccJS_0.2.0_amd64.AppImage">AppImage</a> |
       <a href="https://github.com/alyalin/PraccJS/releases/download/0.2.0/PraccJS_0.2.0_amd64.deb">deb</a>)
      ·
		Windows (<a href="https://github.com/alyalin/PraccJS/releases/download/0.2.0/PraccJS_0.2.0_x64_en-US.msi">msi</a>)
    <br />
    <br />
  </p>
</div>

<br/>

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