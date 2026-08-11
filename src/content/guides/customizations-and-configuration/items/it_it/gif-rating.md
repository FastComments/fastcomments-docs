[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Per impostazione predefinita, il widget dei commenti FastComments imposterà una `gif rating` di `pg`.

Le opzioni disponibili sono `g`, `pg`, `pg-13` e `r`.

Questo può essere impostato nel codice o tramite l'interfaccia utente. Nel codice possiamo farlo come segue:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Imposta Valutazione Gif'; code-example-end]

Nell'interfaccia utente, lo troverai sotto `Gif Picker Rating` finché `Disable Image Uploads?` non è selezionato.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Menu a discesa Gif Picker Rating nella pagina di personalizzazione del widget che offre g, pg, pg-13 e r'; title='Impostazione della valutazione Gif' app-screenshot-end]