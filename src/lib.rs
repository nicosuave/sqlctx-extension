use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
};
use std::fs;
use std::path::Path;

struct SQLContextExtension;

// compiler complaining otherwise
fn main() {
    // Stub main function
}

impl zed::Extension for SQLContextExtension {
    fn new() -> Self {
        SQLContextExtension
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "sqlctx" => {
                if args.is_empty() {
                    return Err("No table name provided".to_string());
                }

                let table_name = &args[0];
                let file_path = Path::new("./sqlctx/").join(format!("{}.yml", table_name));

                if !file_path.exists() {
                    return Err(format!("No YML file found for table {}", table_name));
                }

                let content = fs::read_to_string(file_path)
                    .map_err(|e| format!("Failed to read file: {}", e))?;

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..content.len()).into(),
                        label: format!("SQL Context for {}", table_name),
                    }],
                    text: content,
                })
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "sqlctx" => {
                let mut completions = Vec::new();
                let dir = Path::new("./sqlctx");
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(file_name) = entry.file_name().to_str() {
                                if file_name.ends_with(".yml") && file_name != "combined.yml" {
                                    let table_name = file_name.trim_end_matches(".yml").to_string();
                                    completions.push(SlashCommandArgumentCompletion {
                                        label: table_name.clone(),
                                        new_text: table_name,
                                        run_command: true,
                                    });
                                }
                            }
                        }
                    }
                }
                Ok(completions)
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(SQLContextExtension);
