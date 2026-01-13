---
Per impostazione predefinita, ogni utente può inviare fino a `5 comments` nello stesso minuto.

Questo viene tracciato tramite user id, anon user id e ip address (hashato).

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Nota che, se stai usando l'API per la creazione dei commenti, potresti voler passare l'originale `ip` dell'utente nella richiesta al nostro backend in modo che il rate limiting venga applicato
per utente e non globalmente al tuo account.

---