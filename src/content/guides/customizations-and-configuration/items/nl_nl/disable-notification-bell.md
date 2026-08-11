[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments een meldingsbel in de rechterbovenhoek van het reactiegebied.

Deze bel wordt rood en toont een telling van het aantal meldingen dat de gebruiker heeft. Enkele voorbeeldmeldingen zijn:

- Gebruiker heeft op je gereageerd.
- Gebruiker heeft gereageerd in een thread waarin je hebt gereageerd.
- Gebruiker heeft je reactie upvoted.
- Gebruiker heeft gereageerd op een pagina waarop je geabonneerd bent.

De meldingsbel biedt ook een mechanisme om je op een volledige pagina te abonneren.

We kunnen de meldingsbel echter volledig uitschakelen:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Dit kan ook zonder code worden gedaan. Op de widget‑aanpassingspagina, zie de "Disable Notification Bell" sectie.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Widget‑aanpassingspagina met het selectievakje Disable Notification Bell aangevinkt'; title='Meldingsbel uitschakelen' app-screenshot-end]