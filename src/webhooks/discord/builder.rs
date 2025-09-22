use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DiscordEmbedAuthor {
  pub name: Option<String>,
  pub url: Option<String>,
  pub icon_url: Option<String>
}

impl DiscordEmbedAuthor {
  pub fn new () -> Self {
    Self {
      name: None,
      url: None,
      icon_url: None
    }
  }

  pub fn set_name (&mut self, name: impl Into<String>) -> &mut Self {
    self.name = Some(name.into());
    self
  }

  pub fn set_url (&mut self, url: impl Into<String>) -> &mut Self {
    self.url = Some(url.into());
    self
  }

  pub fn set_icon_url (&mut self, icon_url: impl Into<String>) -> &mut Self {
    self.icon_url = Some(icon_url.into());
    self
  }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DiscordEmbedField {
  pub name: Option<String>,
  pub value: Option<String>,
  pub inline: Option<bool>
}

impl DiscordEmbedField {
  pub fn new () -> Self {
    Self {
      name: None,
      value: None,
      inline: None
    }
  }

  pub fn set_name (&mut self, name: impl Into<String>) -> &mut Self {
    self.name = Some(name.into());
    self
  }

  pub fn set_value (&mut self, value: impl Into<String>) -> &mut Self {
    self.value = Some(value.into());
    self
  }

  pub fn set_inline (&mut self, inline: impl Into<bool>) -> &mut Self {
    self.inline = Some(inline.into());
    self
  }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DiscordEmbedImage {
  pub url: Option<String>
}

impl DiscordEmbedImage {
  pub fn new () -> Self {
    Self {
      url: None
    }
  }

  pub fn set_url (&mut self, url: impl Into<String>) -> &mut Self {
    self.url = Some(url.into());
    self
  }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DiscordEmbedFooter {
  pub text: Option<String>,
  pub icon_url: Option<String>
}

impl DiscordEmbedFooter {
  pub fn new () -> Self {
    Self {
      text: None,
      icon_url: None
    }
  }

  pub fn set_text (&mut self, text: impl Into<String>) -> &mut Self {
    self.text = Some(text.into());
    self
  }

  pub fn set_icon_url (&mut self, icon_url: impl Into<String>) -> &mut Self {
    self.icon_url = Some(icon_url.into());
    self
  }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DiscordEmbed {
  pub author: Option<DiscordEmbedAuthor>,
  pub fields: Option<Vec<DiscordEmbedField>>,
  pub title: Option<String>,
  pub description: Option<String>,
  pub url: Option<String>,
  pub color: Option<u32>,
  pub image: Option<DiscordEmbedImage>,
  pub timestamp: Option<String>,
  pub thumbnail: Option<DiscordEmbedImage>,
  pub footer: Option<DiscordEmbedFooter>
}

impl DiscordEmbed {
  pub fn new () -> Self {
    Self {
      author: None,
      fields: None,
      title: None,
      description: None,
      url: None,
      color: None,
      image: None,
      timestamp: None,
      thumbnail: None,
      footer: None
    }
  }

  pub fn set_author (&mut self, author: impl Into<DiscordEmbedAuthor>) -> &mut Self {
    self.author = Some(author.into());
    self
  }

  pub fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
    self.title = Some(title.into());
    self
  }

  pub fn set_description(&mut self, description: impl Into<String>) -> &mut Self {
    self.description = Some(description.into());
    self
  }

  pub fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
    self.url = Some(url.into());
    self
  }

  pub fn set_color(&mut self, color: impl Into<u32>) -> &mut Self {
    self.color = Some(color.into());
    self
  }

  pub fn set_image(&mut self, image: impl Into<DiscordEmbedImage>) -> &mut Self {
    self.image = Some(image.into());
    self
  }

  pub fn set_timestamp(&mut self, timestamp: impl Into<String>) -> &mut Self {
    self.timestamp = Some(timestamp.into());
    self
  }

  pub fn set_thumbnail(&mut self, thumbnail: impl Into<DiscordEmbedImage>) -> &mut Self {
    self.thumbnail = Some(thumbnail.into());
    self
  }

  pub fn add_field(&mut self, field: DiscordEmbedField) -> &mut Self {
    match &mut self.fields {
      Some(fields) => fields.push(field),
      None => self.fields = Some(vec![field]),
    }
    self
  }

  pub fn set_footer(&mut self, footer: impl Into<DiscordEmbedFooter>) -> &mut Self {
    self.footer = Some(footer.into());
    self
  }
}

const SUPPRESS_EMBEDS_FLAG: u16 = 4;
const SUPPRESS_NOTIFICATIONS_FLAG: u16 = 4096;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DiscordMessage {
  pub content: Option<String>,
  pub username: Option<String>,
  pub avatar_url: Option<String>,
  pub thread_name: Option<String>,
  pub flags: Option<u16>,
  pub embeds: Option<Vec<Option<DiscordEmbed>>>
}

impl DiscordMessage {
  pub fn new () -> Self {
    Self {
      content: None,
      username: None,
      avatar_url: None,
      thread_name: None,
      flags: None,
      embeds: None
    }
  }

  pub fn set_content (&mut self, content: impl Into<String>) -> &mut Self {
    self.content = Some(content.into());
    self
  }

  pub fn set_username (&mut self, username: impl Into<String>) -> &mut Self {
    self.username = Some(username.into());
    self
  }

  pub fn set_avatar_url (&mut self, avatar_url: impl Into<String>) -> &mut Self {
    self.avatar_url = Some(avatar_url.into());
    self
  }

  pub fn set_thread_name (&mut self, thread_name: impl Into<String>) -> &mut Self {
    self.thread_name = Some(thread_name.into());
    self
  }

  pub fn set_suppress_embeds (&mut self, suppress_embeds: impl Into<bool>) -> &mut Self {
    let current_flags = self.flags.unwrap_or(0);

    self.flags = if suppress_embeds.into() {
      if current_flags == SUPPRESS_NOTIFICATIONS_FLAG || current_flags == 0 {
        Some(current_flags + SUPPRESS_EMBEDS_FLAG)
      } else {
        self.flags
      }
    } else {
      if current_flags == SUPPRESS_NOTIFICATIONS_FLAG + SUPPRESS_EMBEDS_FLAG || current_flags == SUPPRESS_EMBEDS_FLAG {
        let new_flags = current_flags - SUPPRESS_EMBEDS_FLAG;

        if new_flags == 0 {
          None
        } else {
          Some(new_flags)
        }
      } else {
        self.flags
      }
    };
    self
  }

  pub fn set_suppress_notifications (&mut self, suppress_notifications: impl Into<bool>) -> &mut Self {
    let current_flags = self.flags.unwrap_or(0);

    self.flags = if suppress_notifications.into() {
      if current_flags == SUPPRESS_EMBEDS_FLAG || current_flags == 0 {
        Some(current_flags + SUPPRESS_NOTIFICATIONS_FLAG)
      } else {
        self.flags
      }
    } else {
      if current_flags == SUPPRESS_NOTIFICATIONS_FLAG + SUPPRESS_EMBEDS_FLAG || current_flags == SUPPRESS_NOTIFICATIONS_FLAG {
        let new_flags = current_flags - SUPPRESS_NOTIFICATIONS_FLAG;

        if new_flags == 0 {
          None
        } else {
          Some(new_flags)
        }
      } else {
        self.flags
      }
    };
    self
  }

  pub fn add_embed (&mut self, embed: impl Into<DiscordEmbed>) -> &mut Self {
    match &mut self.embeds {
      Some(inner_embeds) => inner_embeds.push(Some(embed.into())),
      None => self.embeds = Some(vec![Some(embed.into())])
    }

    self
  }
}