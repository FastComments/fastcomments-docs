[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments chiederà all'utente solo il suo commento, il suo nome utente e la sua email.

Tuttavia, in alcune situazioni potresti voler permettere all'utente di lasciare un link al proprio blog o sito web.

È possibile abilitare la visualizzazione di un campo di input aggiuntivo per inserire l'URL del sito web dell'utente impostando il flag **enableCommenterLinks** su true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Quando viene fornito tale URL, l'account dell'utente verrà aggiornato e il suo nome utente in tutti i commenti passati e futuri sarà collegato a questo URL.

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---