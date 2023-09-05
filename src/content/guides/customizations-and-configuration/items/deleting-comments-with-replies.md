By default, users can delete their own comments. Also, deleting their comment automatically
deletes all child and transient comments in the thread. This behavior is also live.

You can restrict this in the following ways:

- Instead, anonymize the deleted comment (set name and text to `[deleted]` or a custom value).
- Don't allow deleting comments when there are replies. An error message is shown.
- Restrict deleting when a comment has replies to only administrators and moderators.

This can be configured via the `Comment Thread Deletion` section in the Widget Customization UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]
