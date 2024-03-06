By default, FastComments does not limit languages used to comment. 

It may be desirable to limit the languages a community uses.

This can be configured without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

The system will parse their comment and determine its language, and then match it with the allowed list.

If the comment is written in a language that is not allowed, then a localized error message is shown. 
