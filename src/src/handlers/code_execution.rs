use teloxide::prelude::*;
use tokio::process::Command;

pub async fn handle(bot: AutoSend<Bot>, message: Message, code: String) -> ResponseResult<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(code)
        .output()
        .await;

    match output {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            bot.send_message(message.chat.id, format!("Output:\n{}", result))
                .await?;
        }
        Err(e) => {
            bot.send_message(message.chat.id, format!("Error executing code: {}", e))
                .await?;
        }
    }

    Ok(())
}
