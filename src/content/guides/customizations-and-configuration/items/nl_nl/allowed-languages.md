By default, FastComments does not limit languages used to comment. 

It may be desirable to limit the languages a community uses.

This can be configured without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Selector voor toegestane talen op de widget-aanpassingspagina om te beperken welke talen reacties mogen gebruiken'; title='Toegestane talen' app-screenshot-end]

The system will parse their comment and determine its language, and then match it with the allowed list.

If the comment is written in a language that is not allowed, then a localized error message is shown.