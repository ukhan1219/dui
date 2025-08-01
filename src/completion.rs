use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Editor, Helper};
use rustyline::history::DefaultHistory;
use std::borrow::Cow::{self, Borrowed, Owned};
use colored::Colorize;
use crate::docker::DockerClient;

pub struct DockerCompleter {
    docker_client: DockerClient,
}

impl DockerCompleter {
    pub fn new(docker_client: DockerClient) -> Self {
        Self { docker_client }
    }

    fn get_commands() -> Vec<&'static str> {
        vec![
            "containers", "images", "networks", "volumes", "monitor", "interactive",
            "list", "start", "stop", "restart", "pause", "unpause", "remove", "logs", "exec", "inspect", "create", "size", "info",
            "pull", "build", "tag", "push", "stats", "system", "events",
            "help", "exit", "quit", "back"
        ]
    }

    fn get_container_names(&self) -> Vec<String> {
        match self.docker_client.list_containers() {
            Ok(containers) => containers.into_iter().map(|c| c.name).collect(),
            Err(_) => Vec::new(),
        }
    }

    fn get_image_names(&self) -> Vec<String> {
        match self.docker_client.list_images() {
            Ok(images) => images.into_iter().map(|i| format!("{}:{}", i.repository, i.tag)).collect(),
            Err(_) => Vec::new(),
        }
    }
}

impl Completer for DockerCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let mut completions = Vec::new();
        let line_before_cursor = &line[..pos];
        let words: Vec<&str> = line_before_cursor.split_whitespace().collect();

        if words.is_empty() {
            // Complete commands
            for cmd in Self::get_commands() {
                completions.push(Pair {
                    display: cmd.to_string(),
                    replacement: cmd.to_string(),
                });
            }
            return Ok((0, completions));
        }

        if words.len() == 1 {
            // Complete main commands
            let partial = words[0];
            for cmd in Self::get_commands() {
                if cmd.starts_with(partial) {
                    completions.push(Pair {
                        display: cmd.to_string(),
                        replacement: cmd.to_string(),
                    });
                }
            }
            return Ok((0, completions));
        }

        if words.len() == 2 {
            let command = words[0];
            let partial = words[1];

            match command {
                "containers" => {
                    // Complete container subcommands
                    let subcommands = vec!["list", "start", "stop", "restart", "pause", "unpause", "remove", "logs", "exec", "inspect", "create", "size", "info"];
                    for subcmd in subcommands {
                        if subcmd.starts_with(partial) {
                            completions.push(Pair {
                                display: subcmd.to_string(),
                                replacement: subcmd.to_string(),
                            });
                        }
                    }
                }
                "images" => {
                    // Complete image subcommands
                    let subcommands = vec!["list", "pull", "build", "tag", "push", "remove"];
                    for subcmd in subcommands {
                        if subcmd.starts_with(partial) {
                            completions.push(Pair {
                                display: subcmd.to_string(),
                                replacement: subcmd.to_string(),
                            });
                        }
                    }
                }
                "monitor" => {
                    // Complete monitor subcommands
                    let subcommands = vec!["stats", "system", "events"];
                    for subcmd in subcommands {
                        if subcmd.starts_with(partial) {
                            completions.push(Pair {
                                display: subcmd.to_string(),
                                replacement: subcmd.to_string(),
                            });
                        }
                    }
                }
                _ => {}
            }
            return Ok((line_before_cursor.rfind(' ').unwrap_or(0) + 1, completions));
        }

        if words.len() == 3 {
            let command = words[0];
            let subcommand = words[1];
            let partial = words[2];

            match (command, subcommand) {
                ("containers", "start") | ("containers", "stop") | ("containers", "restart") | 
                ("containers", "pause") | ("containers", "unpause") | ("containers", "remove") | 
                ("containers", "logs") | ("containers", "inspect") | ("containers", "info") => {
                    // Complete container names
                    for name in self.get_container_names() {
                        if name.starts_with(partial) {
                            completions.push(Pair {
                                display: name.clone(),
                                replacement: name,
                            });
                        }
                    }
                }
                ("containers", "exec") => {
                    // Complete container names for exec
                    for name in self.get_container_names() {
                        if name.starts_with(partial) {
                            completions.push(Pair {
                                display: name.clone(),
                                replacement: name,
                            });
                        }
                    }
                }
                ("images", "pull") | ("images", "remove") | ("images", "push") => {
                    // Complete image names
                    for name in self.get_image_names() {
                        if name.starts_with(partial) {
                            completions.push(Pair {
                                display: name.clone(),
                                replacement: name,
                            });
                        }
                    }
                }
                ("images", "build") => {
                    // Complete common build paths
                    let paths = vec![".", "./", "../", "~/"];
                    for path in paths {
                        if path.starts_with(partial) {
                            completions.push(Pair {
                                display: path.to_string(),
                                replacement: path.to_string(),
                            });
                        }
                    }
                }
                ("images", "tag") => {
                    // Complete image names for tagging
                    for name in self.get_image_names() {
                        if name.starts_with(partial) {
                            completions.push(Pair {
                                display: name.clone(),
                                replacement: name,
                            });
                        }
                    }
                }
                _ => {}
            }
            return Ok((line_before_cursor.rfind(' ').unwrap_or(0) + 1, completions));
        }

        if words.len() == 4 {
            let command = words[0];
            let subcommand = words[1];
            let partial = words[3];

            match (command, subcommand) {
                ("containers", "exec") => {
                    // Complete common commands for exec
                    let commands = vec!["ls", "ps", "cat", "echo", "pwd", "whoami", "date"];
                    for cmd in commands {
                        if cmd.starts_with(partial) {
                            completions.push(Pair {
                                display: cmd.to_string(),
                                replacement: cmd.to_string(),
                            });
                        }
                    }
                }
                ("images", "tag") => {
                    // Complete common tag formats
                    let tags = vec!["latest", "v1.0", "v1.1", "stable", "dev"];
                    for tag in tags {
                        if tag.starts_with(partial) {
                            completions.push(Pair {
                                display: tag.to_string(),
                                replacement: tag.to_string(),
                            });
                        }
                    }
                }
                _ => {}
            }
            return Ok((line_before_cursor.rfind(' ').unwrap_or(0) + 1, completions));
        }

        Ok((pos, completions))
    }
}

impl Hinter for DockerCompleter {
    type Hint = String;
}

impl Highlighter for DockerCompleter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        // Simple syntax highlighting
        if line.starts_with("containers") {
            Owned(line.to_string().cyan().to_string())
        } else if line.starts_with("images") {
            Owned(line.to_string().green().to_string())
        } else if line.starts_with("networks") {
            Owned(line.to_string().blue().to_string())
        } else if line.starts_with("volumes") {
            Owned(line.to_string().magenta().to_string())
        } else if line.starts_with("monitor") {
            Owned(line.to_string().yellow().to_string())
        } else {
            Borrowed(line)
        }
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Owned(prompt.to_string().cyan().bold().to_string())
    }
}

impl Validator for DockerCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for DockerCompleter {}

pub fn create_editor(docker_client: DockerClient) -> rustyline::Result<Editor<DockerCompleter, DefaultHistory>> {
    let completer = DockerCompleter::new(docker_client);
    let mut editor = Editor::new()?;
    editor.set_helper(Some(completer));
    Ok(editor)
} 