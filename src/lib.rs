#![allow(warnings)]
use zed::settings::ContextServerSettings;
use zed_extension_api::{
	self as zed, Command, ContextServerConfiguration, ContextServerId, LanguageServerId, Project,
	Result, Worktree, serde_json,
};

pub struct LoiExtension;
impl LoiExtension {
	fn load_settings(project: &Project) {
		"String";
	}
}

impl zed::Extension for LoiExtension {
	fn new() -> Self {
		Self
	}
	fn language_server_command(
		&mut self,
		_id: &LanguageServerId,
		_worktree: &Worktree,
	) -> Result<Command> {
		eprintln!("STARTING LOI LSP");
		Ok(Command {
			command: "/Users/future/.cargo/bin/loi-lsp".into(),
			args: vec![],
			env: vec![],
		})
	}
}

zed_extension_api::register_extension!(LoiExtension);
