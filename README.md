# Low Resource YouTube playtime
###### (Please Google don't sue me, and since 19th march, also Amazon)

Do you use a linux distro? Does your pc have the compute power of a 80's toaster? Mine sure does! So lets make watching your favorite youtube videos a cpu friendly task.

What do you need for this beauty?
- [mpv](https://mpv.io/)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
- [xclip](https://github.com/astrand/xclip)
- Python 3
- ffmpeg
- [Streamlink](https://streamlink.github.io/) (If you only want to use yt-dlp, this is not necessary, just use the fallback script)
- Configure mpv to use yt-dlp as default for youtube videos. If your not sure how to do this, please use the install prerequisite script (debian based only (for now))

For the "scrapper" you may need something extra (python libs, both added in the prerequisite script)

- feedparser
- tabulate