[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments consentirà all'utente di inserire un commento con quante righe desidera, fino al limite di caratteri predefinito.

Tuttavia, potrebbe essere opportuno limitare l'utente a inserire una sola riga di testo. Alcuni esempi di casi d'uso includono aste online o chat in tempo reale, per i quali FastComments può essere utilizzato.

Abilitiamo il flag **useSingleLineCommentInput** come segue:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Questo può essere fatto anche senza codice. Nella pagina di personalizzazione del widget, vedere la sezione "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Casella di spunta per l\'input di commenti a riga singola attivata nella pagina di personalizzazione del widget, limitando l\'input a una sola riga'; title='Abilita input di commenti a riga singola' app-screenshot-end]

Nota che i commenti su ogni pagina per ogni direzione di ordinamento sono pre‑calcolati, quindi tutte le direzioni di ordinamento hanno le stesse prestazioni.