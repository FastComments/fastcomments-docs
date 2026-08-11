[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Di default, FastComments mostrerà una campanella di notifica in alto a destra dell'area dei commenti.

Questa campanella diventerà rossa e mostrerà un conteggio del numero di notifiche che l'utente ha. Alcuni esempi di notifiche sono:

- L'utente ti ha risposto.
- L'utente ha risposto in una discussione a cui hai commentato.
- L'utente ha votato positivamente il tuo commento.
- L'utente ha risposto a una pagina a cui sei iscritto.

La campanella di notifica fornisce anche un meccanismo per iscriversi a un'intera pagina, come pure.

Tuttavia, possiamo disabilitare completamente la campanella di notifica:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disabilita la campanella di notifica'; code-example-end]

Questo può essere fatto anche senza codice. Nella pagina di personalizzazione del widget, vedi la sezione "Disabilita la campanella di notifica".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Pagina di personalizzazione del widget con la casella Disabilita la campanella di notifica selezionata'; title='Disabilita la campanella di notifica' app-screenshot-end]