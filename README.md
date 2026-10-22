# Rust - An Introduction

Presentation (about 120 minutes) for beginners without prior Rust knowledge, built with [Slidev](https://sli.dev).
The slides are plain Markdown and run in the browser.

## Getting Started

```bash
npm install
npm run dev        # opens http://localhost:3030
```

| Command          | Purpose                                                     |
| ---------------- | ----------------------------------------------------------- |
| `npm run dev`    | Start the presentation with live reload                     |
| `npm run build`  | Build a static website in `dist/` (e.g. for hosting)        |
| `npm run export` | Export a PDF (`slides-export.pdf`)                          |

Useful shortcuts during the talk:

- `→` / `Space`: next step, `←`: back
- `o`: slide overview
- `d`: toggle dark/light (please keep it dark)
- `f`: fullscreen
- Presenter mode with timer and next-slide preview: <http://localhost:3030/presenter>

## Structure

```text
slides.md             # title, agenda and chapter includes
pages/
  01-why-rust.md      # one file per chapter
  02-tooling.md
  03-basics.md
  04-ownership.md
  05-data-types.md    # structs, enums, pattern matching
  06-traits.md
  07-errors.md
  08-concurrency.md
  09-ecosystem.md
  10-conclusion.md
styles/index.css      # colors (Monokai Pro palette), cards, timeline
setup/monokai-pro.json  # syntax highlighting in Monokai Pro style
setup/shiki.ts        # loads the theme
global-top.vue        # footer with chapter name and slide number
public/ferris.svg     # Ferris (rustacean.net, public domain/CC0)
public/code/          # place for code screenshots from the IDE
code-examples/        # Cargo project with all code examples from the slides
```

## Editing Slides

- Slides are separated by `---`. Options (e.g. `layout`, `class`) go in the block right after the `---`.
- Speaker notes can be added as HTML comments `<!-- ... -->` at the end of a slide. They show up in presenter mode.
- `<v-clicks>` and `v-click` reveal content step by step.
- Highlight code lines step by step: ` ```rust {all|1-3|5} `
- Slide too full? Set `class: dense` in the frontmatter. This shrinks code and text, the title keeps its size.
- To shorten the talk, hide slides with `hide: true` in the frontmatter.
- Helper classes from `styles/index.css`: `card`, `muted`, `small-code`, `dense`, `chapter`, `c-orange`, `c-green`,
  `c-blue`, ...
- Tailwind-style layout utilities (UnoCSS) work directly, e.g. `grid grid-cols-2 gap-8`.
- Text on slides is selectable (`selectable: true` in `slides.md`, CSS rule in `styles/index.css`).
  Please do not use `select-none` or `user-select: none`.

To add a new chapter, create a file in `pages/` and include it in `slides.md`:

```md
---
src: ./pages/11-new-chapter.md
---
```

## Code: Live or as a Screenshot

Code blocks on the slides are already rendered with a Monokai Pro-like color scheme (`setup/monokai-pro.json`). This
keeps them editable and searchable.

If you prefer screenshots from the IDE (Monokai Pro):

1. Save the screenshot as a PNG in `public/code/`, e.g. `public/code/ownership.png`
2. Replace the code block on the slide with:

   ```md
   ![Ownership](/code/ownership.png)
   ```

   Or with a fixed width: `<img src="/code/ownership.png" class="w-3/4" />`

The code background on the slides (`#2d2a2e`) matches the Monokai Pro editor background, so screenshots blend in with
the live code blocks.

## Code Examples

All examples from the slides are in `code-examples/examples/` and compile (tested with Rust 1.99). The number at the
start of the file name is the chapter. Handy for IDE screenshots or live demos:

```bash
cd code-examples
cargo run --example 05_enums
cargo run --example 09_axum      # web server: curl localhost:3000/me
cargo run --example 08_channels  # threads send their results through std::sync::mpsc
cargo run --example 08_channels_tokio  # the same idea with tokio::sync::mpsc and .await
cargo run --example 08_shared_state  # threads with Arc<Mutex<T>>
cargo run --example 09_clap -- --help   # reorder <SKU> --quantity
cargo run --example 07_panic -- unwrap  # panics shown in chapter 7 (index, unwrap, expect)
cargo run --example 07_handling # four ways to handle a Result, ends with the expect panic
cargo run --example 10_order_api # combined example: curl localhost:3000/orders/2
cargo test              # library tests and doc tests
cargo test --examples   # also runs the tests inside the examples (e.g. 03_modules)
```

`code-examples/compile-errors/` contains the slide examples that intentionally do not compile. They are not part of
the Cargo build and produce the error messages shown on the slides, e.g.:

```bash
rustc --edition 2024 code-examples/compile-errors/04_borrow_error.rs
rustc --edition 2024 code-examples/compile-errors/08_data_race.rs  # E0499, two threads write the same Vec
rustc --edition 2024 code-examples/compile-errors/03_private_function/main.rs  # multi-file example
```
