use anyhow::Result;
use gtk::prelude::*;
use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{Component, ComponentParts, ComponentSender, RelmApp};
use std::sync::OnceLock;

use crate::overlay::{OverlayBackend, OverlayCommand, OverlayEvent, OverlayState};

static OVERLAY_SENDER: OnceLock<ComponentSender<Overlay>> = OnceLock::new();

fn overlay_sender() -> Result<&'static ComponentSender<Overlay>> {
    OVERLAY_SENDER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Overlay sender not initialized"))
}

#[derive(Debug)]
struct Overlay {
    overlay_state: Option<OverlayState>,
}

pub struct Relm4OverlayBackend {
    app_handle: tokio::task::JoinHandle<()>,
}

impl OverlayBackend for Relm4OverlayBackend {
    fn new() -> Result<Relm4OverlayBackend> {
        const ARGS: [String; 0] = [];
        let app_handle = tokio::spawn(async {
            RelmApp::new("intelli-ocr.overlay")
                .with_args(ARGS.to_vec())
                .run::<Overlay>(());
        });

        Ok(Relm4OverlayBackend { app_handle })
    }

    fn update(&mut self, command: OverlayCommand) -> Result<()> {
        let sender = overlay_sender()?;
        let _ = sender
            .command_sender()
            .send(command)
            .map_err(|_| anyhow::anyhow!("Failed to send command to overlay"))?;
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let sender = overlay_sender()?;
        let _ = sender
            .command_sender()
            .send(OverlayCommand::Event(OverlayEvent::EscapePressed))
            .map_err(|_| anyhow::anyhow!("Failed to send close command to overlay"))?;
        Ok(())
    }
}

#[relm4::component]
impl Component for Overlay {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = OverlayCommand;

    view! {
      #[root]
      gtk::Window {
          set_title: Some("Intelli-OCR Overlay"),
          set_decorated: true,
          set_resizable: false,

          gtk::Label {
              set_label: "Overlay Content",
          },

          // gtk::Button {
          //     set_label: "Close",
          //     connect_clicked => {
          //         sender.command_sender().send(OverlayCommand::Event(OverlayEvent::EscapePressed)).unwrap();
          //     }
          // }
      }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        root.init_layer_shell();
        root.set_layer(Layer::Overlay);
        root.set_fullscreened(true);
        root.set_visible(false);

        let _ = OVERLAY_SENDER.set(sender);

        let model = Overlay {
            overlay_state: None,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            OverlayCommand::ReplaceState(state) => {
                self.overlay_state = Some(state);
                root.set_visible(true);
            }
            OverlayCommand::UpdateSelection(_selection) => {
                // Update selection in the model and refresh the view if needed
            }
            OverlayCommand::ClearSelection => {
                // Clear selection in the model and refresh the view if needed
            }
            OverlayCommand::Event(event) => match event {
                OverlayEvent::EscapePressed => {
                    root.set_visible(false);
                    self.overlay_state = None;
                }
                OverlayEvent::CopyPressed => {
                    // Handle copy action
                }
                OverlayEvent::DragStarted(_point) => {
                    // Handle drag start
                }
                OverlayEvent::DragUpdated(_point) => {
                    // Handle drag update
                }
                OverlayEvent::DragFinished(_point) => {
                    // Handle drag finish
                }
            },
        }
    }
}
