[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Het maximale aantal tekens dat in het reactie‑invoerveld mag worden ingevoerd, kan worden beperkt door de **maxCommentCharacterLength** parameter.

Standaard is 2000.

Dingen zoals afbeeldings‑URL's worden niet meegerekend bij de lengtebepaling.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Beperk commentaargrootte'; code-example-end]

Dit kan zonder code worden aangepast op de widget‑aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Maximale commentaargrootte veld op de widget‑aanpassingspagina, gebruikt om te beperken hoeveel tekens een commentaar kan bevatten'; title='Beperk commentaargrootte' app-screenshot-end]