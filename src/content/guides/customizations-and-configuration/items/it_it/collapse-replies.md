[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, le risposte ai commenti di primo livello vengono mostrate.

Questo può essere configurato in modo che l'utente debba cliccare "Mostra risposte" sui commenti di primo livello per vedere i commenti figli.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Questa impostazione non influenzerà il numero di commenti di primo livello caricati inizialmente. Se hai un commento di primo livello e 29 commenti figli, con questa impostazione attivata vedrai:

- Vedrai il commento di primo livello.
- Vedrai Mostra risposte (29) sotto questo commento.

Se desideri mostrare tutti i commenti di primo livello in combinazione con questa opzione, imposta [starting page to -1](#starting-page).