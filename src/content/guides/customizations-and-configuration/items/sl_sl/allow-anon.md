By default, FastComments will require an email to comment. It does not have to be a valid email, however until the user clicks a link sent to them,
their comment will display an "Unverified Comment" label.

However, we can remove the email requirement. The email input field will still show, but it will no longer be required.

This can be configured via the widget customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.allow-anonymous-comments'; alt='Možnost anonimnih komentarjev v uporabniškem vmesniku prilagajanja gradnika, ki naredi polje za e‑pošto neobvezno'; title='Omogočanje anonimnih komentarjev' app-screenshot-end]

A username is still required. To remove that step as well, you can
[nastavite privzeto uporabniško ime](/guide-customizations-and-configuration.html#default-username) that everyone shares, or have FastComments
[ustvari edinstveno uporabniško ime](/guide-customizations-and-configuration.html#auto-generate-username) for each visitor.