By default, FastComments will require an email to comment. It does not have to be a valid email, however until the user clicks a link sent to them,
their comment will display an "Unverified Comment" label.

However, we can remove the email requirement. The email input field will still show, but it will no longer be required.

This can be configured via the widget customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.allow-anonymous-comments'; alt='Anonieme reacties optie in de widget-aanpassings-UI, die het e-mailveld optioneel maakt'; title='Anonieme reacties inschakelen' app-screenshot-end]

A username is still required. To remove that step as well, you can
[een standaard gebruikersnaam instellen](/guide-customizations-and-configuration.html#default-username) that everyone shares, or have FastComments
[een unieke gebruikersnaam genereren](/guide-customizations-and-configuration.html#auto-generate-username) for each visitor.