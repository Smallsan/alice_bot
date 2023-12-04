# Alice Bot (Still Work In Progress!!) (Like Seriously)

A MultiPurpose Discord Bot Coded In Rust.

## Admin Features

- [x] Stalker Mode - Have you ever wanted to stalk a special someone in your discord server? You just need their ```UserID``` and you're good to go.
  
- [x] Channel Event Logger - It just logs events that happen inside your servers into a text channel, Just provide a ```ChannelID```. *(It's Kinda Like Dyno)*
  
- [x] Local Event Logger - It's just like the Channel Event Logger but instead of saving it in a channel, It saves it locally in a text file. *(One Text File Per Day Dear Lord...)*
  
- [x] Attachment Downloader - It saves all the attachments that anyone sends in the server to your local drive. *(It Doesn't Filter Out NSFW...)*
  
- [ ] Anti NSFW Filter - Uses a trained AI model to detect and auto delete explicit images sent in your server's non NSFW channels.
  
- [ ] Record Everything Everywhere - It logs every single message sent in every single channel in the server, It saves it in separated text files named after the channel names.
  
- [ ] Record Everything Here - Records every single message but in just a specific channel.

## User Features

- [x] Bubble Wrap - It generates that generic spoiler bubble wrap message in discord.
  
- [x] Backtrack - It shows recently sent messages in the channel.

- [ ] Reputation Tracker - It allows you to have a reputation system in your server, And be able to display that.

- [ ] Booru Search - It allows you to fetch images from either danbooru or safebooru.
  
## How To Run

1. Create a `keys.json` inside the `config folder`.

2. Place your discord api key inside the `keys.json`.

    ```json
    {
        "discord_api_key": "Your discord api key",
    }
    ```

3. Configure the `config.json` inside the `config folder`.

    ```json
    {
        "log_channel_id": "Channel_Id of where you want logs to be sent",
        "msg_stalker_user_id": "User_Id of the one you want to stalk",
        "msg_stalker_receiver_id": "User_Id of the receiver of the stalk messages"
    }
    ```

4. Either run the project by typing `cargo run --release` in the terminal or compile it and run it the same way.

## Common Problems

### Message Stalker Isn't Sending Me Messages

- Make sure you allow ```Direct messages from server members``` in your discord account.
  
- Be sure to check if you correctly set the values ```msg_stalker_user_id``` and ```msg_stalker_receiver_id``` in the config file.

## Contribution

- Be sure to format the code with ```cargo fmt``` before creating a pull request.
