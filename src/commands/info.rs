use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "```diff
-Information-
Owner: Xanthus
Language: Rust
Framework: Serenity
Description: A small little bot I 'Xanthus#3862' use to experiment with the serenity discord bot framework for rust```".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info")
        .description("Displays information about Malacyte")
}
