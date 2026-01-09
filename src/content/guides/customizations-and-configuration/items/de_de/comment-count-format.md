[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Die oben im Kommentar-Widget angezeigte Kommentaranzahl kann angepasst werden.

Dies kann durch beliebigen Text ersetzt werden, und der Wert **[count]** wird durch die Anzahl ersetzt, für den Benutzer lokalisiert.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Dies kann ohne Code auf der Seite zur Anpassung des Widgets vorgenommen werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]