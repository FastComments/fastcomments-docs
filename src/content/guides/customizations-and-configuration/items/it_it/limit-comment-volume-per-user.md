---
Per impostazione predefinita, ogni utente può inviare fino a `5 commenti` nello stesso minuto.

Questo viene tracciato tramite ID utente, ID utente anonimo e indirizzo IP (hashato).

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Campo Max commenti per minuto nella pagina di personalizzazione del widget, impostato a 5 per impostazione predefinita'; title='Limitare il volume dei commenti per utente' app-screenshot-end]

Nota che se stai usando l'API di creazione dei commenti potresti voler passare l'indirizzo `ip` originale dell'utente nella richiesta al nostro backend affinché il limitatore di velocità venga applicato
per utente e non globalmente al tuo account.

---