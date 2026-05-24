use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    registry::{RegistryState, SimpleGlobal},
    shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer, LayerShell},
    shm::{Shm, slot::SlotPool},
};
use wayland_client::{
    Connection, EventQueue, QueueHandle,
    globals::{GlobalList, registry_queue_init},
};

pub struct WaylandCompositorBackend {
    conn: Connection,
    globals: GlobalList,
    event_queue: EventQueue<Self>,
    registry_state: RegistryState,
}

impl WaylandCompositorBackend {
    pub fn new() -> Result<Self> {
        let conn = Connection::connect_to_env()?;
        let (globals, mut event_queue) = registry_queue_init(&conn)?;
        let qh = event_queue.handle();

        let compositor = CompositorState::bind(&globals, &qh)?;
        let layer_shell = LayerShell::bind(&globals, &qh)?;
        let shm = Shm::bind(&globals, &qh)?;

        let surface = compositor.create_surface(&qh);
        let layer =
            layer_shell.create_layer_surface(&qh, surface, Layer::Top, Some("simple_layer"), None);

        layer.set_anchor(Anchor::BOTTOM);
        layer.set_keyboard_interactivity(KeyboardInteractivity::OnDemand);
        layer.set_size(0, 0);

        layer.commit();

        let pool = SlotPool::new(256 * 256 * 4, &shm)?;

        // let registry_state = RegistryState::new(&globals);
        // let output_state = OutputState::new(&globals, &qh);

        let mut backend = Self {
            conn,
            globals,
            event_queue,
            registry_state,
        };

        backend.event_queue.roundtrip(&mut backend);
        backend.event_queue.roundtrip(&mut backend);

        Ok(backend)
    }
}

impl CompositorBackend for WaylandCompositorBackend {
    fn get_active_output(&self) -> Result<String> {
        let outputs = self
            .registry_state
            .list::<SimpleGlobal<wl_output::WlOutput>>()?;

        Ok("".to_string())
    }
}
