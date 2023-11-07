use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;

pub async fn message_stalker(msg: &Message) {
    if msg.author.bot {
        return;
    }
    let stalker_channel_id: ChannelId = ChannelId(967685973456609320);


}
