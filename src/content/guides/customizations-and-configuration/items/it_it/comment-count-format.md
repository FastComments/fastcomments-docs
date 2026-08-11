[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Il conteggio dei commenti visualizzato nella parte superiore del widget dei commenti può essere personalizzato.

Questo può essere sostituito con qualsiasi stringa, e il valore **[count]** verrà sostituito con il valore del conteggio, localizzato per l'utente.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Personalizzazione del testo del conteggio dei commenti'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Campo di testo del conteggio dei commenti nella pagina di personalizzazione del widget, dove [count] è sostituito con il totale in tempo reale'; title='Personalizzazione del testo del conteggio dei commenti' app-screenshot-end]