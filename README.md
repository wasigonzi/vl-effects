# VL Effects

**VL Effects** es la herramienta de motion graphics y efectos visuales de **Visualab Studios**:
composiciones, capas, fotogramas clave, expresiones, máscaras, rotoscopia, texto animado y
exportación, en el escritorio (Windows, macOS, Linux) y en el navegador.

- Web: https://www.visualabstudios.com/vl-tools/vl-effects
- Visualab Studios: https://www.visualabstudios.com

## Compilar

```sh
cargo run -p effectcraft --release    # app de escritorio
cargo xtask web --serve 8765          # versión web en http://127.0.0.1:8765/
```

Los nombres internos de los paquetes de Rust (`effectcraft-*`), el formato de proyecto (`.ecproj`)
y los comandos de automatización se mantienen por compatibilidad. Documentación técnica:
[`docs/`](docs/) y [`packaging/`](packaging/).

## Licencia

VL Effects se basa en software de código abierto:
Copyright (c) 2026 ArtCraft Team and the EffectCraft contributors. Licensed under the MIT License or
the Apache License, Version 2.0, at your option ([LICENSE-MIT](LICENSE-MIT),
[LICENSE-APACHE](LICENSE-APACHE)). Los avisos requeridos están en [NOTICE](NOTICE) y el material
de terceros en [ATTRIBUTION.md](ATTRIBUTION.md).

El nombre y los logotipos de Visualab Studios son marcas de Visualab Studios y no están cubiertos
por la licencia anterior.
