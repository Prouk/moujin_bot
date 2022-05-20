# Moujin Bot
Rust implementation of moujin bot
## Documentation

- [serenity](https://github.com/serenity-rs/serenity)
- [songbird](https://github.com/serenity-rs/songbird)

This bot requires `opus-driver`, `youtube-dowload` and `ffmpeg` to be installed on the machine,and accessible via the terminal in order to play music.

Currently, 3 commands are working on this bot : 

- `/join <voice channel>` to join a voice channel before playing an audio stream.
- `/play <youtube url>` to play the youtube video.
- `/character <server> <character name>` to fetch and display basic info about your FFXIV character.


## Explication

I created this bot when rythm disappeared, for me and my friends.
But i don't like message spamming in a channel, so with this bot every commands refer to a `player`, wich is the first play command.

Every info will always be on that specific message, and it will be updated each time a music end.
same thing for the commands, there is no `skip` or `stop` commands, every interraction is made with buttons on the `player`.

At the end of the queue, the player will disappear until we add another song.

## Authors

- [@Prouk](https://www.github.com/prouk)

