---
[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments consentirà all'utente di inserire un commento su quante righe desidera, fino al limite di caratteri predefinito.

Tuttavia, potrebbe essere desiderabile limitare l'utente a inserire solo una singola riga di testo. Alcuni casi d'uso esemplificativi includono offerte online, o chat in tempo reale, per le quali FastComments
può essere utilizzato.

Attiviamo il **useSingleLineCommentInput** flag come segue:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Questo può anche essere fatto senza codice. Nella pagina di personalizzazione del widget, vedere la sezione "Abilita l'immissione di commenti su singola riga".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Nota che, i commenti su ogni pagina per ogni direzione di ordinamento sono precalcolati, quindi tutte le direzioni di ordinamento hanno le stesse prestazioni.

---