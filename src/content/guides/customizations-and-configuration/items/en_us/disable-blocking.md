[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

By default, FastComments lets users block other users. Blocking a user masks their comments,
prevents notifications between the users, and so on.

You may want to disable this feature. You can do it like this:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

This can also be done without code, which also enables proper server-side validation, via the Widget Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]