[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Standaard heeft FastComments live commentaar ingeschakeld.

Dit betekent dat elke kijker van de commentaarthread dezelfde inhoud moet zien.

Bijvoorbeeld, als een commentaar wordt toegevoegd, moet dat commentaar worden weergegeven. Als een commentaar wordt bewerkt of verwijderd,
dan zullen die commentaren voor alle kijkers van de thread worden bewerkt of verwijderd. Hetzelfde geldt voor stemmen en alle moderatieacties.

We kunnen dit echter uitschakelen:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Live commentaar uitschakelen'; code-example-end]

Dit kan ook zonder code worden gedaan. Op de widget-aanpassingspagina, zie de sectie "Live commentaar uitschakelen".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Sectie \'Live commentaar uitschakelen\' van de widget-aanpassingspagina, waarmee realtime thread-updates worden uitgeschakeld'; title='Live commentaar uitschakelen' app-screenshot-end]

---