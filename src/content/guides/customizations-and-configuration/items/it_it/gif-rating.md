[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Per impostazione predefinita, il widget dei commenti FastComments imposterà un `gif rating` di `pg`.

Le opzioni disponibili sono `g`, `pg`, `pg-13` e `r`.

Questo può essere impostato nel codice o tramite l'interfaccia utente. Nel codice possiamo farlo come segue:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

Nell'interfaccia utente, troverai questo sotto `Gif Picker Rating` purché `Disable Image Uploads?` non sia selezionato.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]