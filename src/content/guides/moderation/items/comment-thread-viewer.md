When moderating and viewing comment threads it is desirable to be able to jump directly to a thread to get context during moderating.

This means that the user's flow starts in the Comment Moderation page, and would then have to go from an individual comment to
the page containing that comment, wait for that page to load, wait for the comments to load, and then scroll to that comment.

However, FastComments provides a faster way. In the Moderate Comments page, next to each comment, there is a "View Comment" button in the bottom right.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.content .comment'; title='A Comment' app-screenshot-end]

If this comment has replies, the button text will instead say the number of replies, but clicking it takes the same action.

This button will take you to the **Comment Thread Viewer**.

The Comment Thread Viewer is a small, fast loading application hosted by FastComments that renders the comment thread for the page that
the comment is on, and scrolls to that comment.

This allows moderators to gather the context they need to, quickly, without having to wait for another page to load.
