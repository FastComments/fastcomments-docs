[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Standaard houdt FastComments niet bij wie elke reactie heeft bekeken of geeft enige statistieken hierover.

We kunnen deze functie echter inschakelen, waarna het systeem begint bij te houden wanneer elke gebruiker naar een reactie scrolt.

Wanneer dit gebeurt, wordt een teller naast een oogpictogram bij elke reactie verhoogd. De teller wordt live bijgewerkt en afgekort volgens de lokale instellingen van de gebruiker.

We kunnen dit inschakelen door de vlag **enableViewCounts** op true te zetten:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Dit kan zonder code worden aangepast op de pagina voor het aanpassen van de widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

We houden de gebruikers-id* bij die de reactie heeft bekeken, zodat wanneer je de reactie opnieuw bekijkt deze niet wordt verhoogd. Als je de reactie pas na twee jaar opnieuw bekijkt, wordt de teller wél weer verhoogd.

- *Opmerking: of de anonieme sessie-id, of het IP-adres van de gebruiker als een gehashte waarde.