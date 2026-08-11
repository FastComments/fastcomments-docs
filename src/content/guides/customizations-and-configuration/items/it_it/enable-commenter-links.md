[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments chiederà all'utente solo il commento, il nome utente e l'email.

Tuttavia, in alcune situazioni potresti voler che l'utente lasci un collegamento al proprio blog o sito web.

Possiamo abilitare la visualizzazione di un campo di input aggiuntivo per inserire l'URL del sito web dell'utente impostando il flag **enableCommenterLinks** su true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Abilitare i collegamenti dei commentatori'; code-example-end]

Quando tale URL viene fornito, l'account dell'utente verrà aggiornato e tutti i loro nomi utente su tutti i commenti passati e futuri saranno collegati a questo URL.

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Pagina di personalizzazione del widget con la casella dei collegamenti dei commentatori selezionata per aggiungere un campo URL del sito web al modulo di commento'; title='Abilitare i collegamenti dei commentatori' app-screenshot-end]