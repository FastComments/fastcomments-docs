By default, users can delete their own comments. Deleting a comment also automatically deletes all child and transient comments in the thread. This behavior is live.

You can restrict this in the following ways:

- Instead, anonymize the deleted comment (set name and text to `[deleted]` or a custom value).
- Disallow deleting a comment when it has replies. A customizable error message is displayed.
- Restrict deletion of comments that have replies to administrators and moderators only.

This can be configured via the `Comment Thread Deletion` section in the Widget Customization UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---