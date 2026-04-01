use bevy::prelude::*;
use pybevy::pybevy_core::{self, PyPlugin, plugin::plugin_registry};
use pyo3::prelude::*;

// Custom module, needs to be named as _pybevy since pybevy expects the native module by such name
#[pymodule(gil_used = false)]
fn _pybevy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pybevy::init_module(m)?;

    // custom classes
    plugin_registry::register_plugin_bridge(RotatePluginBridge);
    m.add_class::<PyRotatePlugin>()?;

    Ok(())
}

#[derive(Component)]
pub struct Rotating {
    pub speed: f32,
}

pub struct RotatePlugin {
    pub speed: f32,
}

impl Default for RotatePlugin {
    fn default() -> Self {
        Self { speed: 1.0 }
    }
}

impl Plugin for RotatePlugin {
    fn build(&self, app: &mut App) {
        let speed = self.speed;
        app.add_systems(
            Startup,
            move |mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<StandardMaterial>>| {
                setup_scene(&mut commands, &mut meshes, &mut materials, speed);
            },
        );
        app.add_systems(Update, rotate_system);
    }
}

fn setup_scene(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    speed: f32,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Rotating { speed },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        })),
    ));
}

fn rotate_system(mut query: Query<(&mut Transform, &Rotating)>, time: Res<Time>) {
    for (mut transform, rotating) in &mut query {
        transform.rotate_y(rotating.speed * time.delta_secs());
    }
}

#[pyclass(name = "RotatePlugin", extends = PyPlugin, frozen)]
pub struct PyRotatePlugin {
    speed: f32,
}

#[pymethods]
impl PyRotatePlugin {
    #[new]
    #[pyo3(signature = (speed = 1.0))]
    fn new(speed: f32) -> (Self, PyPlugin) {
        (PyRotatePlugin { speed }, PyPlugin)
    }
}

// TODO: PyBevy should expose all below, so that this custom scaffolding would not be needed
pub struct RotatePluginBridge;

impl pybevy_core::PluginBridge for RotatePluginBridge {
    fn py_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<PyRotatePlugin>()
    }

    fn py_type_ptr(&self) -> *const pyo3::ffi::PyTypeObject {
        pyo3::Python::attach(|py| {
            <PyRotatePlugin as pyo3::PyTypeInfo>::type_object(py).as_type_ptr()
        })
    }

    fn py_type<'py>(&self, py: pyo3::Python<'py>) -> pyo3::Bound<'py, pyo3::types::PyType> {
        <PyRotatePlugin as pyo3::PyTypeInfo>::type_object(py)
    }

    fn name(&self) -> &'static str {
        "RotatePlugin"
    }

    fn build(
        &self,
        py_plugin: &pyo3::Bound<'_, pyo3::PyAny>,
        app: &mut bevy::app::App,
    ) -> PyResult<()> {
        let plugin = py_plugin.extract::<pyo3::PyRef<'_, PyRotatePlugin>>()?;
        app.add_plugins(RotatePlugin {
            speed: plugin.speed,
        });
        Ok(())
    }
}
