[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

By default, FastComments has live commenting enabled.

This means that every viewer of the comment thread sees the same content.

For example, if a comment is added, that comment will appear. If a comment is edited or removed,
then those comments will be edited or removed for all viewers of the thread. The same goes for votes and all moderation actions.

However, you can disable this:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

You can also do this without code. On the widget customization page, see the "Disable Live Commenting" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]