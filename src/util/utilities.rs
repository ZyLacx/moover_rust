use std::sync::Arc;

use serenity::{all::{GuildId, ChannelId}, http::Http};

use anyhow::Context;

pub async fn get_system_channel(guild_id: GuildId, http: Arc<Http>) -> anyhow::Result<ChannelId> {
    return http.get_guild(guild_id).await?.system_channel_id
        .context(format!("System channel of guild: {} not found", guild_id.get()));
}