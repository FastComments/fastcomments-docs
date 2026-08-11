[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Standaard stelt de FastComments-reactie-widget een `gif rating` in op `pg`.

Beschikbare opties zijn `g`, `pg`, `pg-13` en `r`.

Dit kan worden ingesteld in de code of via de UI. In de code kunnen we dit als volgt doen:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

In de UI vind je dit onder `Gif Picker Rating` zolang `Disable Image Uploads?` niet is aangevinkt.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Gif Picker Rating vervolgkeuzelijst op de widget-aanpassingspagina met g, pg, pg-13 en r'; title='Instellen van de Gif Rating' app-screenshot-end]