///! This package collets all functionality meant to run as web servers
/// 

use super::*;

pub mod api;
pub mod webui;

// from .api import main as run_api_server
// from .webui import main as run_webui_server
pub use api::main as run_api_server;
pub use webui::main as run_webui_server;



//
