---
[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Standaard staat FastComments gebruikers toe andere gebruikers te blokkeren. Het blokkeren van een gebruiker zorgt ervoor dat hun reacties worden gemaskeerd, voorkomt notificaties tussen de gebruikers, enzovoort.

Het kan wenselijk zijn deze functionaliteit uit te schakelen. Dit kan als volgt worden gedaan:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Dit kan ook zonder code worden gedaan, wat ook zorgt voor correcte validatie aan de serverzijde, via de Widget-aanpassingsinterface:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---