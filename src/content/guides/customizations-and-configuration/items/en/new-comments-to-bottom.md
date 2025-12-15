[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

By default, new live comments appear at the top of the comment list as they are posted in real-time.

When this option is enabled, new live comments will be added to the bottom of the list instead. This affects how comments appear when they are posted live while users are viewing the comment thread.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

With this setting enabled:
- New live comments posted by other users will appear at the bottom of the comment list
- Users will see new comments appear below existing comments in real-time
- This only affects live comment updates - not the initial page load
- This can help maintain reading flow when users are following a discussion

Note that this setting only affects where new live comments are placed as they arrive in real-time. It does not affect the initial sort order when the page loads.