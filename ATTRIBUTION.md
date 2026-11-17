# Attribution

Every non-code asset in VL Effects, with its author, source and licence. Each repository file listed here also has
a `<file>.attribution` sidecar with full details. The rules are in [AGENTS.md](AGENTS.md) §1: no Adobe
iconography, images or other Adobe assets; open licences only; every asset attributed.
`cargo xtask assets` checks that this index and the sidecars cover every asset in the repository.
The complete licence texts and copyright notices for the bundled fonts, including the fonts embedded
by the pinned egui dependency, are reproduced in [`NOTICE`](NOTICE), which every package ships; the
`assets/fonts/OFL-*.txt` files are the upstream copies kept beside the fonts.

## Bundled dependency fonts

These fonts are embedded by the pinned egui dependency (epaint_default_fonts 0.37 at
`6d518fc799681571963842f95b55a6b68431b15a`); no font file is added to this repository.
egui-icons is always embedded, `default_fonts` adds Ubuntu Light and Hack, and
`monochrome_emoji_fonts` adds Noto Emoji and emoji-icon-font. The complete copyright notices
and licence texts are in [`NOTICE`](NOTICE).

| Asset | Author | Source | Licence |
|---|---|---|---|
| `epaint_default_fonts/fonts/egui-icons.ttf` | John Slegers (emoji-icon-font) | https://github.com/storytold/egui/blob/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts/egui-icons.txt; original https://github.com/jslegers/emoji-icon-font | MIT (complete copyright and permission text in `NOTICE`) |
| `epaint_default_fonts/fonts/emoji-icon-font.ttf` | John Slegers | https://github.com/storytold/egui/blob/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts/emoji-icon-font-mit-license.txt; original https://github.com/jslegers/emoji-icon-font | MIT (complete copyright and permission text in `NOTICE`) |
| `epaint_default_fonts/fonts/NotoEmoji-Regular.ttf` | Google Inc. (The Noto Project Authors) | https://github.com/storytold/egui/blob/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts/OFL.txt | OFL-1.1 (complete text in `NOTICE`) |
| `epaint_default_fonts/fonts/Ubuntu-Light.ttf` | Canonical Ltd. and Dalton Maag | https://github.com/storytold/egui/blob/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts/UFL.txt | Ubuntu Font Licence 1.0 (complete text in `NOTICE`) |
| `epaint_default_fonts/fonts/Hack-Regular.ttf` | Source Foundry Authors; Bitstream Inc. | https://github.com/storytold/egui/blob/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts/Hack-Regular.txt; original https://github.com/source-foundry/Hack | MIT AND Bitstream-Vera (complete texts in `NOTICE`) |

## Asset files

| Asset | Author | Source | Licence |
|---|---|---|---|
| `assets/fonts/Inter-Regular.ttf` | The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) |
| `assets/fonts/Inter-Medium.ttf` | The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) |
| `assets/fonts/Inter-SemiBold.ttf` | The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) |
| `assets/fonts/Inter-Bold.ttf` | The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) |
| `assets/fonts/Inter-Italic.ttf` | The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) |
| `assets/fonts/NotoSerif-Regular.ttf` | The Noto Project Authors | https://github.com/notofonts/latin-greek-cyrillic | OFL-1.1 (`assets/fonts/OFL-NotoSerif.txt`) |
| `assets/fonts/NotoSansSymbols2-Regular.ttf` | The Noto Project Authors | https://github.com/notofonts/symbols | OFL-1.1 (`assets/fonts/OFL-NotoSansSymbols2.txt`) |
| `assets/fonts/JetBrainsMono-Regular.ttf` | The JetBrains Mono Project Authors | https://github.com/JetBrains/JetBrainsMono | OFL-1.1 (`assets/fonts/OFL-JetBrainsMono.txt`) |
| `apps/effectcraft-web/web/favicon.svg` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `packaging/macos/dmg/background.svg` | @XusBadia | Original work: macOS DMG Finder window background, derived from the VL Effects app icon (`assets/app-icon/effectcraft.svg`); rendered to a TIFF by `packaging/macos/package.sh` at package time | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) |
| `assets/app-icon/effectcraft-1024.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/effectcraft-macos-512.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/effectcraft-small.svg` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/effectcraft.icns` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/effectcraft.ico` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/effectcraft.svg` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/128x128/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/16x16/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/24x24/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/256x256/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/32x32/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/48x48/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/512x512/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/64x64/apps/ai.storyteller.effectcraft.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/app-icon/hicolor/scalable/apps/ai.storyteller.effectcraft.svg` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `crates/lottie/tests/fixtures/bouncing-ball.json` | VL Effects contributors | Original work: hand-written Lottie test fixture (bouncing ball, eased position keys) | MIT OR Apache-2.0 |
| `crates/lottie/tests/fixtures/shapes-gradient.json` | VL Effects contributors | Original work: hand-written Lottie test fixture (star, rectangle, gradient fill, dashed stroke, trim paths) | MIT OR Apache-2.0 |
| `crates/lottie/tests/fixtures/precomp-matte-text.json` | VL Effects contributors | Original work: hand-written Lottie test fixture (precomp, track matte, parented text) | MIT OR Apache-2.0 |
| `crates/lottie/tests/fixtures/legacy-masks.json` | VL Effects contributors | Original work: hand-written Lottie test fixture (legacy keyframes, masks) | MIT OR Apache-2.0 |
| `crates/svg/tests/fixtures/basic-shapes.svg` | VL Effects contributors | Original work: hand-written SVG test fixture (basic shapes, groups, transforms) | MIT OR Apache-2.0 |
| `crates/svg/tests/fixtures/paths-gradients.svg` | VL Effects contributors | Original work: hand-written SVG test fixture (path data, gradients, CSS, use) | MIT OR Apache-2.0 |
| `crates/model/tests/fixtures/quad.gltf` | VL Effects contributors | Original work: glTF 2.0 test fixture (textured quad, node hierarchy, translation animation; data URIs) | MIT OR Apache-2.0 |
| `crates/model/tests/fixtures/quad.glb` | VL Effects contributors | Original work: binary glTF test fixture (the same quad with its texture in the BIN chunk) | MIT OR Apache-2.0 |
| `crates/model/tests/fixtures/cube.obj` | VL Effects contributors | Original work: Wavefront OBJ test fixture (unit cube, two materials) | MIT OR Apache-2.0 |
| `crates/model/tests/fixtures/cube.mtl` | VL Effects contributors | Original work: MTL test fixture (materials of cube.obj) | MIT OR Apache-2.0 |

Three small After Effects test projects of `crates/aep` are in this repository: `crates/aep/tests/fixtures/synthetic.aep` (original work, made with After Effects 2026 from own data, working space None, the embedded colour profile removed; MIT OR Apache-2.0, with its `.attribution` sidecar), `crates/aep/tests/fixtures/ai-mask-3099.aep` (original work, made with After Effects 2026 from four hand-written one-page PDFs named `.ai`, footage paths and host name neutralised, the embedded colour profiles replaced by stubs made with `crates/aep/examples/stubicc.rs`; MIT OR Apache-2.0, with its `.attribution` sidecar) and `crates/aep/tests/fixtures/text-synthetic.aep` (original work, made with After Effects 2026 by script from own data, working space None, no colour profile embedded, nothing changed after the save; MIT OR Apache-2.0, with its `.attribution` sidecar). The other After Effects test projects of `crates/aep` are not in this repository. They are original work of an VL Effects contributor, generated with After Effects 2026 as own test data (MIT OR Apache-2.0, each file with its `.attribution` sidecar); they are in [storytold/effectcraft-corpus](https://github.com/storytold/effectcraft-corpus) with their sidecars, fetched by `cargo xtask aep-corpus` (commit-pinned, SHA-256-verified), and the tests read them only from the folder named by `EFFECTCRAFT_AEP_FIXTURES` (opt-in). Their embedded colour profiles are replaced by stubs made with `crates/aep/examples/stubicc.rs` (a header and a description, no colour data), so no vendor profile data is in them.

## Screenshots

Screenshots of VL Effects itself, rendered headlessly, showing only procedurally generated demo content.

| Asset | Author | Source | Licence |
|---|---|---|---|
| `docs/images/effectcraft-hero.png` | VL Effects contributors | Original work: screenshot of VL Effects (VL Effects main window) | MIT OR Apache-2.0 |
| `docs/images/effectcraft-3d.png` | VL Effects contributors | Original work: screenshot of VL Effects (VL Effects Classic 3D) | MIT OR Apache-2.0 |
| `docs/images/effectcraft-graph-editor.png` | VL Effects contributors | Original work: screenshot of VL Effects (VL Effects Graph Editor) | MIT OR Apache-2.0 |
| `docs/images/effectcraft-effects.png` | VL Effects contributors | Original work: screenshot of VL Effects (VL Effects effects) | MIT OR Apache-2.0 |
| `docs/images/effectcraft-render-queue.png` | VL Effects contributors | Original work: screenshot of VL Effects (VL Effects Render Queue panel with the demo compositions queued for H.264 and ProRes export) | MIT OR Apache-2.0 |

## Code-drawn assets (original work, MIT OR Apache-2.0)

- UI icons drawn in code: `crates/ui-egui/src/icons.rs`, including the original filled favorite star (2026-10-11, VL Effects contributors).
- Procedural demo content (solids, shapes, gradients, text animations): `crates/engine/src/demo.rs`.
- Built-in project templates (Lower Third, Title Card, Logo Reveal, Kinetic Type, Social Square, Vertical Story,
  Slideshow, 3D Text Orbit) built from engine commands, and their thumbnails rendered by our own renderer:
  `crates/engine/src/templates.rs`.
- Text animator presets (Typewriter, Fade Up Characters, Bounce In Words, Tracking In, Scramble, Blur In, Jitter,
  Drop In Lines): `crates/engine/presets/text_animators.json` (sidecar `.attribution`).
- Transition animation presets (23: wipes, clock and fan reveals, blinds, dissolves, card flips, CC wipes, fades):
  `crates/engine/presets/transitions.json` (sidecar `.attribution`).
- Backgrounds animation presets (23: drifting gradients, fog, cloud and smoke, plasma and magma, cells and crystals, scrolling grids
  and checks, pulse rings, bokeh, light sweep): `crates/engine/presets/backgrounds.json` (sidecar `.attribution`).
- Behaviors animation presets (22: wiggle, shake, float, orbit, bounce, drift, pendulum, spin, pulse, heartbeat, blink, strobe, flicker):
  `crates/engine/presets/behaviors.json` (sidecar `.attribution`).
- Sample scripts (File ▸ Scripts) and ScriptUI panel: `crates/engine/scripts/Rename Layers.jsx`,
  `crates/engine/scripts/Sort Layers by In Point.jsx`, `crates/engine/scripts/Create Null at Selected Layers.jsx`,
  `crates/engine/scripts/Render Queue Batch.jsx`, `crates/engine/scripts/ScriptUI Panels/Layer Tools.jsx`
  (sidecars `.attribution`).
- Bundled extension (ScriptUI panel) with its twelve built-in ease presets:
  `extensions/scriptui-panels/Ease Presets.jsx` (sidecar `.attribution`).
- Example effect plug-in: `examples/plugins/posterize-bands` (WebAssembly ABI 1).

## Trained models (optional downloads, not bundled)

The repository and the installers contain no model weights. These models are downloaded only when
a user asks (Settings ▸ Roto Brush, Settings ▸ Face Tracking, or the `roto.model.*` /
`face.model.*` commands), unmodified from their authors' official location, checked against a
pinned SHA-256 and stored in the user's `models` folder with a `<file>.NOTICE.txt` naming their
authors, licence and source. VL Effects runs them with its own Rust implementation and is not
affiliated with or endorsed by their authors.

| Model | Authors | Source | Licence |
|---|---|---|---|
| MobileSAM (`mobile_sam.pt`) | Chaoning Zhang et al., building on Meta AI's Segment Anything and Microsoft's TinyViT | https://github.com/ChaoningZhang/MobileSAM | Apache-2.0 |
| MediaPipe Face Landmarker (`face_landmarker.task`: BlazeFace short-range detector, Face Mesh V2, blendshapes) | Google LLC | https://developers.google.com/edge/mediapipe/solutions/vision/face_landmarker (model cards: https://storage.googleapis.com/mediapipe-assets/MediaPipe%20BlazeFace%20Model%20Card%20(Short%20Range).pdf, https://storage.googleapis.com/mediapipe-assets/Model%20Card%20MediaPipe%20Face%20Mesh%20V2.pdf) | Apache-2.0 |

## Formats and parameters from other open-source projects

Written in Rust for VL Effects; no code is copied. They follow published specifications and
configuration from these Apache-2.0 projects:

- `crates/segment/src/tflite.rs` reads TensorFlow Lite files using the field layout of TensorFlow
  Lite's `schema.fbs` (The TensorFlow Authors, https://github.com/tensorflow/tensorflow,
  Apache-2.0).
- `crates/segment/src/mediapipe.rs` follows MediaPipe's published face landmarker graph: the SSD
  anchor layout, detection decoding, crop sizes and the face-mesh point indices it uses (Google
  LLC, https://github.com/google-ai-edge/mediapipe, Apache-2.0).
- `crates/segment/src/mobilesam.rs` follows the MobileSAM, TinyViT and Segment Anything
  architectures and their Apache-2.0 reference code.

## Frozen matte regression fixture

| Asset | Author | Source | Licence |
|---|---|---|---|
| `crates/engine/src/tests/fixtures/analysed-0.24.ecproj` | arnav-hiwarkar | Original work, synthetic project keyed by VL Effects 0.24.1 | MIT OR Apache-2.0 |
| `crates/engine/src/tests/fixtures/object-matte-0.22.ecproj` | lisn0 | Original work, synthetic project saved by VL Effects 0.22.0 | MIT OR Apache-2.0 |
| `crates/engine/src/tests/fixtures/tracked-0.22.ecproj` | arnav-hiwarkar | Original work, synthetic project saved by VL Effects 0.22.0 | MIT OR Apache-2.0 |

## Visualab Studios brand

The Visualab Studios name and logos are trademarks of Visualab Studios. The artwork below is
original work by Visualab Studios.

| File | Author | Source | Licence |
|---|---|---|---|
| `assets/app-icon/visualab-mark.png` | Visualab Studios | Original work: VL Effects app icon (Visualab Studios brand mark) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) |
| `assets/brand/visualab-splash-bg.png` | Visualab Studios | Original work: Visualab Studios loading-screen background | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) |
| `assets/brand/visualab-splash-logo.png` | Visualab Studios | Original work: Visualab Studios loading-screen artwork | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) |
| `apps/effectcraft-web/web/vl-logo.png` | Visualab Studios | Original work: Visualab Studios brand mark (web loading screen) | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) |
| `apps/effectcraft-web/web/favicon.png` | Visualab Studios | Original work: Visualab Studios brand mark; 128 px | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) |
| `apps/effectcraft-web/web/icon-180.png` | Visualab Studios | Original work: Visualab Studios brand mark; 180 px home-screen icon | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) |
| `apps/effectcraft-web/web/icon-192.png` | Visualab Studios | Original work: Visualab Studios brand mark; 192 px home-screen icon | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) |

- Appearance theme previews, drawn procedurally from original shapes: `crates/ui-egui/src/panels/settings.rs` (original work, MIT OR Apache-2.0).

## Synthetic test audio

| Asset | Author | Source | Licence |
|---|---|---|---|
| `crates/media/tests/fixtures/pce61-ffmpeg.aac-fixture` | Dylan M. Taylor | Original sine waves; reproducible recipe in `crates/media/tests/fixtures/README.md` | MIT OR Apache-2.0 |
