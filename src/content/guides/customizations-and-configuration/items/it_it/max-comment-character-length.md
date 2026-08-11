[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Il numero massimo di caratteri consentiti da inserire nel campo di input del commento può essere limitato dal parametro **maxCommentCharacterLength**.

Il valore predefinito è 2000.

Elementi come gli URL delle immagini non sono inclusi nel calcolo della lunghezza.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Campo della dimensione massima del commento nella pagina di personalizzazione del widget, usato per limitare il numero di caratteri che un commento può contenere'; title='Limita la lunghezza del commento' app-screenshot-end]