use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

// Funzione per registrare il comando "balance".
pub fn register() -> CreateCommand {
    CreateCommand::new("balance").description("Replies with Balance!!")
}

// Funzione principale del comando "balance".
pub async fn run<'a>(_options: &'a [ResolvedOption<'a>]) -> String {
    "Balance".to_string()
}
