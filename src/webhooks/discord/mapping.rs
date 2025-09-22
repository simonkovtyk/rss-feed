use crate::{models::{Channel, Post}, webhooks::discord::{builder::{DiscordEmbed, DiscordEmbedAuthor, DiscordEmbedFooter, DiscordMessage}, config::DiscordConfig, utils::convert_color}};

pub fn map_to_discord_message (config: DiscordConfig, channel: &Channel, post: &Post) -> DiscordMessage {
  let mut message_embed_footer = DiscordEmbedFooter::new();
  let mut message_embed = DiscordEmbed::new();

  if let Some(inner_title) = &post.title {
    message_embed.set_title(inner_title);
  }

  if let Some(inner_content) = &post.content {
    message_embed.set_description(inner_content);
  }

  if let Some(inner_pub_data) = &post.pub_date {
    message_embed.set_timestamp(inner_pub_data);
  }

  if let Some(inner_link) = &post.link {
    message_embed.set_url(inner_link);
  }

  if let Some(inner_style) = &config.style {
    if let Some(embed) = &inner_style.embed {
      message_embed.set_color(
        convert_color(
          &embed
        )
      );
    }
  }

  message_embed_footer.set_text(&channel.name);
  
  if let Some(image_url) = &channel.image_url {
    message_embed_footer.set_icon_url(image_url);
  }

  message_embed.set_footer(message_embed_footer);

  DiscordMessage::new()
    .add_embed(message_embed)
    .clone()
}