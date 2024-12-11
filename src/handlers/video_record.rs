use teloxide::prelude::*;
use tokio::process::Command;

pub async fn handle(bot: AutoSend<Bot>, message: Message, duration: String) -> ResponseResult<()> {
    let duration_secs = duration.parse::<u32>().unwrap_or(0);
    let output_file = "video.mp4";

    let _ = Command::new("ffmpeg")
        .arg("-t")
        .arg(duration_secs.to_string())
        .arg("-f")
        .arg("v4l2")
        .arg("-i")
        .arg("/dev/video0")
        .arg(output_file)
        .output()
        .await;

    bot.send_video(message.chat.id, output_file).await?;
    Ok(())
}
