# pybevy-whl-build

Template for building a custom [pybevy](https://github.com/pybevy/pybevy) wheel with native Rust plugins.

Bundles pybevy + your Rust code into a single `.whl` — all `#[pyclass]` types share one `.so`, avoiding type identity issues across extension modules.

## How it works

1. **Cargo.toml** depends on `pybevy` (rlib) and `bevy`
2. **build.rs** copies pybevy's Python package (.py, .pyi, py.typed) into `OUT_DIR`
3. **pyproject.toml** tells maturin to include those files in the wheel and place the `.so` at `my_app/pybevy/_pybevy.so`
4. **src/lib.rs** calls `pybevy::init_module()` and registers your own types + plugin bridges

## Usage

```bash
python -m venv .venv && source .venv/bin/activate
maturin develop
python examples/scene_3d.py
```

```python
from my_app.pybevy.prelude import *
from my_app import RotatePlugin

@entrypoint
def main(app: App) -> App:
    return app.add_plugins(DefaultPlugins, RotatePlugin(speed=2.0))

if __name__ == "__main__":
    main().run()
```

## Adding your own plugins

See `src/lib.rs` for the pattern: define a Bevy `Plugin`, wrap it in a `#[pyclass(extends = PyPlugin)]`, implement `PluginBridge`, and register both in the `_pybevy` module.

## Limitations

- **`pybevy watch` does not work** — the `pybevy` CLI runs from a separate environment and can't find the `my_app` package. Use `python examples/scene_3d.py` directly instead. See [pybevy/pybevy#58](https://github.com/pybevy/pybevy/issues/58).
