[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Standaard is live reageren in FastComments ingeschakeld.

Dit betekent dat elke kijker van de discussiedraad dezelfde inhoud te zien krijgt.

Bijvoorbeeld, als een opmerking wordt toegevoegd, moet die opmerking zichtbaar zijn. Als een opmerking wordt bewerkt of verwijderd,
dan worden die opmerkingen voor alle kijkers van de draad bewerkt of verwijderd. Hetzelfde geldt voor stemmen, en alle moderatie-acties.

We kunnen dit echter uitschakelen:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Dit kan ook zonder code worden gedaan. Ga op de pagina voor het aanpassen van de widget naar de sectie "Live reageren uitschakelen".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---