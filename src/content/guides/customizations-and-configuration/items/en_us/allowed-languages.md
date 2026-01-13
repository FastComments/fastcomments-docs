By default, FastComments does not limit the languages that can be used to comment. 

You may want to limit the languages a community uses.

This can be configured without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

The system parses each comment to determine its language, then matches it against the allowed list.

If a comment is written in a language that is not allowed, a localized error message is displayed.