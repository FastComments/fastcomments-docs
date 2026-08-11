[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Standaard staat FastComments gebruikers toe om andere gebruikers te blokkeren. Het blokkeren van een gebruiker zorgt ervoor dat hun opmerkingen worden gemaskeerd, voorkomt meldingen tussen de gebruikers, enzovoort.

Het kan wenselijk zijn om deze functionaliteit uit te schakelen. Dit kan als volgt:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Blokkering uitschakelen'; code-example-end]

Dit kan ook zonder code worden gedaan, wat ook een juiste server‑side validatie mogelijk maakt, via de Widget‑aanpassings‑UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Optie om blokkering uit te schakelen in de widget‑aanpassings‑UI, die voorkomt dat gebruikers elkaar blokkeren'; title='Blokkering uitschakelen' app-screenshot-end]

---