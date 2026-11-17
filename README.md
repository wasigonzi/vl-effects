# VL Effects

**VL Effects** es la herramienta de motion graphics y efectos visuales de **Visualab Studios**:
composiciones, capas, fotogramas clave, expresiones, máscaras, rotoscopia, texto animado y
exportación, con 308 efectos integrados, en el escritorio (Windows, macOS, Linux) y en el navegador.

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

MIT OR Apache-2.0. Los términos de la licencia, los avisos de copyright requeridos y el material de
terceros están en [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE), [NOTICE](NOTICE) y
[ATTRIBUTION.md](ATTRIBUTION.md).

El nombre y los logotipos de Visualab Studios son marcas de Visualab Studios y no están cubiertos
por la licencia anterior.
