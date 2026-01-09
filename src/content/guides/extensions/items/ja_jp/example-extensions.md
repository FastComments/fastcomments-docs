---
At FastComments, we write our own extensions, using the same API. You can view the unminified code
for these extensions at the following endpoints:

#### ダークモード

The Dark Mode extension is conditionally loaded when a "dark" page is detected. This extension simply adds
some CSS to the comment widget. This way we do not have to load the dark mode CSS when it is not needed.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### モデレーション

When the current user is a moderator, we use the moderation extension.

This is a good example for adding click-based event listeners, making API requests, adding to the comment menu and comment areas.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### ライブチャット

The Live Chat extension (in combination with other configuration and styling) turns the comment widget into a live chat
component.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js

---