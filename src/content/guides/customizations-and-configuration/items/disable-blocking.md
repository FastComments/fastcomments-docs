[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

By default, FastComments allows users to block other users. Blocking a user will cause their comments
to be masked, prevents notifications between the users, and so on.

It may be desirable to disable this functionality. It can be done like so:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

This can also be done without code, which also enables proper server-side validation, via the Widget Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]
