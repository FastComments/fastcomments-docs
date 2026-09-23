[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Standaard houdt FastComments niet bij wie elk commentaar heeft bekeken en biedt geen statistieken hierover.

We kunnen deze functie echter inschakelen, waarna het systeem begint bij te houden wanneer elke gebruiker naar een commentaar scrolt.

Wanneer dit gebeurt, wordt een teller naast een oog‑icoon dat bij elk commentaar wordt weergegeven, verhoogd. De teller wordt live bijgewerkt en afgekort volgens de locale van de gebruiker.

We kunnen dit inschakelen door de **enableViewCounts**‑vlag op true te zetten:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Dit kan zonder code worden aangepast op de widget‑aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Widget‑aanpassingspagina met het selectievakje voor weergavetellingen aangevinkt, zodat elk commentaar een oog‑icoon en teller toont'; title='Inschakelen van weergavetellingen voor reacties' app-screenshot-end]

We volgen de gebruikers‑id* die het commentaar heeft bekeken gedurende één week, zodat als je het commentaar binnen die week opnieuw bekijkt, de teller niet wordt verhoogd. Als je het commentaar na die week opnieuw bekijkt, wordt de teller weer verhoogd.

- *Opmerking: of de anonieme sessie‑id, of het IP‑adres van de gebruiker als een gehashte waarde.