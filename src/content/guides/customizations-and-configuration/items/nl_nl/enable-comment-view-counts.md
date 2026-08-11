[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Standaard houdt FastComments niet bij wie elke reactie heeft bekeken of biedt enige statistieken hierover.

We kunnen deze functie echter inschakelen, waarna het systeem begint te volgen wanneer elke gebruiker naar een reactie scrolt.

Wanneer dit gebeurt, wordt een teller naast een oogpictogram dat op elke reactie wordt getoond, verhoogd. De teller wordt live bijgewerkt en afgekort volgens de locale van de gebruiker.

We kunnen dit inschakelen door de **enableViewCounts**‑vlag op true te zetten:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Inschakelen van weergavetellingen voor reacties'; code-example-end]

Dit kan zonder code worden aangepast, op de widget‑aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Widget-aanpassingspagina met het selectievakje voor weergavetellingen aangevinkt zodat elke reactie een oogpictogram en teller toont'; title='Inschakelen van weergavetellingen voor reacties' app-screenshot-end]

We volgen de gebruikers‑id* die de reactie heeft bekeken, zodat als je de reactie opnieuw bekijkt, deze niet wordt verhoogd. Als je de reactie opnieuw bekijkt na twee jaar, zal de teller meer verhogen.

- *Opmerking: of de anonieme sessie‑id, of het IP‑adres van de gebruiker als een gehashte waarde.