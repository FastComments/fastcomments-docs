[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Die am oberen Rand des Kommentar-Widgets angezeigte Kommentaranzahl kann angepasst werden.

Dies kann durch beliebige Zeichenfolge ersetzt werden, und der Wert **[count]** wird durch den Zählerwert ersetzt, lokalisiert für den Benutzer.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Anpassen des Kommentarzähltextes'; code-example-end]

Dies kann ohne Code auf der Widget-Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Kommentarzähler-Textfeld auf der Widget-Anpassungsseite, wobei [count] durch die aktuelle Gesamtsumme ersetzt wird'; title='Anpassen des Kommentarzähltextes' app-screenshot-end]