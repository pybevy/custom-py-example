from my_app.pybevy.prelude import *
from my_app import RotatePlugin


@entrypoint
def main(app: App) -> App:
    return app.add_plugins(DefaultPlugins, RotatePlugin(speed=2.0))


if __name__ == "__main__":
    main().run()
