# Low Resource YouTube playtime
###### (Please Google don't sue me, and since 19th march, also Amazon)

Do you use a linux distro? Does your pc have the compute power of a 80's toaster? Mine sure does! So lets make watching your favorite youtube videos a cpu friendly task.

What do you need for this beauty?
- [mpv](https://mpv.io/)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
- ffmpeg
- Configure mpv to use yt-dlp as default for youtube videos. If your not sure how to do this, please use the install prerequisite script.


### Nice! How can I use it?
Well you can blindly trust me and run the following command with sudo.
```
sh -c "$(curl -fsSL https://raw.githubusercontent.com/HFMorais/low-resource-youtube-playtime/main/install-prerequisites.sh)"
```

### And now what?
Now just use the binary, simply call it in your terminal of choice.
```
Usage: command [-h] [-q quality] <URL>
-h: Print this help message.
-q: Specify the video quality (e.g., 720, 1080, 360). If the specified quality is not available, 720 and upwards will be used.
<URL>: URL of the video to be played.
```
My favorite way of using it, is to create a keyboard shortcut and just have the video url in the clipboard.

