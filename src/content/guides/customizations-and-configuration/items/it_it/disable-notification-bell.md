---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments mostrerà una campanella di notifica nell'angolo in alto a destra dell'area dei commenti.

Questa campanella diventerà rossa e mostrerà un conteggio del numero di notifiche dell'utente. Alcuni esempi di notifiche sono:

- Un utente ti ha risposto.
- Un utente ha risposto in una discussione in cui hai commentato.
- Un utente ha votato positivamente il tuo commento.
- Un utente ha risposto a una pagina a cui sei iscritto.

La campanella di notifica fornisce anche un meccanismo per iscriversi a un'intera pagina.

Tuttavia, possiamo disabilitare completamente la campanella di notifica:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Questo può anche essere fatto senza codice. Nella pagina di personalizzazione del widget, consulta la sezione "Disabilita la campanella di notifica".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]
---