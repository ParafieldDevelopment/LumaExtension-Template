//! Example Luma extension. The same implementation is exported as a native
//! DLL/cdylib and as a WASM module (via the `guest` helper module).

use anyhow::Result;
use luma_api::event::EventPriority;
use luma_api::events::{WorkspaceLoadedEvent, WORKSPACE_LOADED_EVENT};
use luma_api::{CoreAPI, ExtensionsCoreAPI, Shared};
use luma_extensions::{LumaExtension, log};

#[derive(Default)]
struct ExampleExtension;

impl LumaExtension for ExampleExtension {
    fn name(&self) -> &str {
        "example-extension"
    }

    fn initialize(
        &mut self,
        core_api: &Shared<dyn CoreAPI>,
        extensions_core: &Shared<dyn ExtensionsCoreAPI>,
    ) -> Result<()> {
        log(3, "Example extension initializing");
        log(
            3,
            &format!("Luma data directory: {}", core_api.get_data_dir().display()),
        );
        log(
            3,
            &format!(
                "Already loaded extensions: {:?}",
                extensions_core.loaded_extension_names()
            ),
        );

        // The guest bus is bridged to the host, so this listener receives the
        // same event that the IDE fires when a workspace finishes loading.
        core_api.event_bus().register::<WorkspaceLoadedEvent, _>(
            "example-extension",
            EventPriority::Normal,
            false,
            |event, _context| {
                log(
                    3,
                    &format!(
                        "Workspace loaded: {} ({})",
                        event.workspace.face().title,
                        event.workspace.face().path
                    ),
                );
            },
        );
        log(3, &format!("Listening for {WORKSPACE_LOADED_EVENT}"));
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        log(3, "Example extension shutting down");
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
luma_extensions::declare_luma_extension!(ExampleExtension);

#[cfg(target_arch = "wasm32")]
luma_extensions::declare_wasm_extension!(ExampleExtension);
