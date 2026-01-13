By default, FastComments will allow users to delete their comments.

However, it is possible to prevent this.

In the widget customization page, see the "Disable Deleting" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- This only impacts regular Commenters and not moderators or admins, who will still be able to delete.
- This will also impact API integrations for when `contextUserId` is passed.