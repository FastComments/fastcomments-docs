[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, le risposte ai commenti di livello superiore vengono visualizzate.

Questo può essere configurato in modo che l'utente debba fare clic su "Show Replies" sui commenti di livello superiore per vedere le risposte.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Comprimi le risposte ai commenti di livello superiore'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Opzione per comprimere le risposte nell\'interfaccia di personalizzazione del widget, nascondendo i commenti figli dietro un link Mostra risposte'; title='Comprimi le risposte' app-screenshot-end]

Questa impostazione non influenzerà il numero di commenti di livello superiore caricati inizialmente. Se hai un commento di livello superiore e 29 risposte, con questa impostazione attiva otterrai:

- Vedi il commento di livello superiore.
- Vedi Show Replies (29) sotto questo commento.

Se desideri mostrare tutti i commenti di livello superiore in combinazione con questa opzione, imposta [pagina iniziale a -1](#starting-page).