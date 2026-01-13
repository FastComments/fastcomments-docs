By default, FastComments allows users to edit their comments.

However, you can prevent this.

On the widget customization page, see the "Disable Editing" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- This only affects regular Commenters and not moderators or admins, who will still be able to edit.
- This will also affect API integrations when `contextUserId` is passed.