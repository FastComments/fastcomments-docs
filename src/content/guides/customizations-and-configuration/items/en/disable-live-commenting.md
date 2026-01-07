[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

By default, FastComments will have live commenting enabled.

This means that every viewer of the comment thread should see the same content.

For example, if a comment is added, that comment should show. If a comment is edited or removed,
then those comments will be edited or removed for all viewers of the thread. Same with votes, and all moderation actions.

However, we can disable this:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

This can also be done without code. In the widget customization page, see the "Disable Live Commenting" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]
