Using FastComments Scheduled Comments is simple. First, you'll want to [import the comments here](https://fastcomments.com/auth/my-account/manage-data/import-scheduled).

This page can also be accessed via `Manage Data->Schedule Comments`.

### Manually Played Comments

*For manually* played comments (you have to manually hit Play), you have the option to start playing back comments. This will play back the comments on each page you've defined in the CSV
file, with the delays between each comment based on the delay you specified.

This is useful when you have a scheduled live webinar that starts at a specific time. When the webinar starts, just hit Play
in the dashboard.

### Automatically Playing Comments

*For automatically* playing comments, the comments are played back on each page load for each user.

This is useful for scenarios where videos or other content starts from the beginning with each pay load. 

### Dynamically Growing Playback

Each time the autoplay script is ran for a user - on page load - there is still the
opportunity for others to comment.

As people leave comments, their comments, the comments are **automatically added to the replay
script** at the same offset from page load, so the conversation continues to grow without any
manual work.

You can additionally **moderate** the comments posted, to curate which comments you want to show
each time the autoplay script is ran.

The `Moderate Comments` page will also show a timestamp like `AutoPlay 1hr 2m 30s` next to each
comment instead of the date.

This is only available for automatic playback, not manually scheduled playback.

### Configuration

Every comment will be posted **live**. You may want to consider [turning on showing live comments right away](/guide-customizations-and-configuration.html#show-live-right-away).

You can learn about the import format in the Import Format section of this documentation.
