# Alice Bot (Still Work In Progress!!)
A MultiPurpose Discord Bot Coded In Rust.

## Functions
- Stalker Mode - Give it **Discord UserID's** and you shall receive *via dms*.
- Channel Event Logger - Give it a **Discord Channel** and it'll record all the events in your server *kinda like dyno*.
- Local Event Logger - This will record all the events in your server and store it in a **Text File** like fresh one *per day*.
- Attachment Downloader - Downloads all the attachments sent in your server *Porn or non Porn* in your **Local Storage**.
- Anti NSFW Filter - Uses AI to detect *Sussy Images* sent in your server and deletes them.
- Record Everything Everywhere - Records every single message in all the channels in the server and saves them in *Text Files*
- Record Everything Here - Records every single message in a specific channel.
- BackTrack - Shows past (?) messages.
- Reputation Tracker - Allows members to do +rep -rep and show reps of members.

## How To Run

1. Create a `keys.json` inside the `config folder`.

2. Place your discord api key inside the `keys.json`.

```json
{
    "discord_api_key": "Your discord api key",
}
```

3. Configure the `config.json` inside the `config folder`.

4. Either run the project by typing `cargo run --release` in a terminal at the directory or compile it and run it the same way.




