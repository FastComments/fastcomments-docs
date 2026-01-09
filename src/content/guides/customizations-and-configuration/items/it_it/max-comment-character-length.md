[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Il numero massimo di caratteri consentito nell'area di inserimento del commento può essere limitato dal parametro **maxCommentCharacterLength**.

Il valore predefinito è 2000.

Elementi come gli URL delle immagini non vengono inclusi nella determinazione della lunghezza.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]