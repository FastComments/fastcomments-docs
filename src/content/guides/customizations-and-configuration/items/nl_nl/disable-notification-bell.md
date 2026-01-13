[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments een notificatiebel rechtsboven in het reactiegebied.

Deze bel wordt rood en toont een teller met het aantal meldingen dat de gebruiker heeft. Enkele voorbeelden van meldingen zijn:

- Een gebruiker heeft op je gereageerd.
- Een gebruiker heeft gereageerd in een thread waarop jij hebt gereageerd.
- Een gebruiker heeft je reactie omhoog gestemd.
- Een gebruiker heeft gereageerd op een pagina waarop je geabonneerd bent.

De notificatiebel biedt ook de mogelijkheid om je op een gehele pagina te abonneren.

We kunnen de notificatiebel echter volledig uitschakelen:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Dit kan ook zonder code worden gedaan. Op de pagina voor widget-aanpassing, zie de sectie "Notificatiebel uitschakelen".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---