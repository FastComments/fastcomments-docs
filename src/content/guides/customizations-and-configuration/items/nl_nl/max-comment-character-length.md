[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Het maximale aantal tekens dat in het invoerveld voor opmerkingen mag worden ingevoerd, kan worden beperkt met de **maxCommentCharacterLength**-parameter.

De standaard is 2000.

Dingen zoals afbeeldings-URL's worden niet meegenomen bij het bepalen van de lengte.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Dit kan zonder code worden aangepast op de pagina voor het aanpassen van de widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---