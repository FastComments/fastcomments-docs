By default, FastComments will allow users to edit their comments.

However, it is possible to prevent this.

In the widget customization page, see the "Disable Editing" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- This only impacts regular Commenters and not moderators or admins, who will still be able to edit.
- This will also impact API integrations for when `contextUserId` is passed. 
