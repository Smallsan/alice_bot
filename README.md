# Alice Bot (Work In Progress - Seriously!)

Alice Bot is a versatile Discord bot developed in Rust.

## Admin Features

- [x] Stalker Mode: Easily stalk a specific user in your Discord server using their `UserID`.
  
- [x] Channel Event Logger: Logs server events into a designated text channel using a provided `ChannelID` (similar to Dyno).
  
- [x] Local Event Logger: Saves server events in a text file locally, one file per day (a bit excessive, right?).
  
- [x] Attachment Downloader: Saves all server attachments to your local drive (NSFW content included).
  
- [ ] Anti NSFW Filter: Aims to auto-delete explicit images in non-NSFW channels using a trained AI model.
  
- [ ] Record Everything Everywhere: Logs every message in every channel, saving them in separate text files by channel name.
  
- [ ] Record Everything Here: Records every message but in a specific channel.

## User Features

- [x] Bubble Wrap: Generates a generic spoiler bubble wrap message in Discord.
  
- [x] Backtrack: Displays recently sent messages in the channel.

- [ ] Reputation Tracker: Sets up a reputation system in your server and enables display.

- [ ] Booru Search: Fetches images from either danbooru or safebooru.

## How To Run

1. Create a `keys.json` file inside the `config` folder.

2. Place your Discord API key inside the `keys.json`:

    ```json
    {
        "discord_api_key": "Your Discord API key"
    }
    ```

3. Configure the `config.json` inside the `config` folder:

    ```json
    {
        "log_channel_id": "Channel ID for logs",
        "msg_stalker_user_id": "UserID to stalk",
        "msg_stalker_receiver_id": "UserID of the recipient for stalk messages"
    }
    ```

4. Run the project by typing `cargo run --release` in the terminal or compile and run it similarly.

## Common Problems

### Message Stalker Isn't Sending Messages

- Ensure you've allowed `Direct messages from server members` in your Discord account settings.
  
- Double-check the accuracy of values set for `msg_stalker_user_id` and `msg_stalker_receiver_id` in the config file.

## Contribution

- Prior to creating a pull request, ensure code formatting with `cargo fmt`.
