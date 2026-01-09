Per SSO esiste la seguente configurazione da considerare per le notifiche:

- Whether the user has opted into notifications.
  - Questo si ottiene impostando il flag `optedInNotifications` su `true` o `false` nell'oggetto `SSOUser`.
  - Questo può essere impostato tramite l'API.
  - Inoltre, se passi un valore per questo flag nel payload, verrà aggiornato automaticamente quando l'utente carica un thread di commenti.
- Whether the user has opted into **subscription** notifications.
  - Questo si ottiene impostando il flag `optedInSubscriptionNotifications` su `true` o `false` nell'oggetto `SSOUser`.
  - Questo può essere impostato tramite l'API.
  - Inoltre, se passi un valore per questo flag nel payload, verrà aggiornato automaticamente quando l'utente carica un thread di commenti.
- Defining their email.
  - Se non presente, non possiamo inviare notifiche via email.
- Whether to disable unsubscribe links in emails.
  - Questo si ottiene tramite il flag `disableUnsubscribeLinks` nell'oggetto `Tenant`.
  - Questo può essere impostato tramite l'API.
- Whether to use a custom unsubscribe link.
  - Questo si ottiene tramite la proprietà `footerUnsubscribeURL` sull'oggetto `DomainConfig`.
  - Questo può essere impostato tramite l'API.
  - Potresti anche voler considerare di impostare gli header di disiscrizione pertinenti tramite `emailHeaders` nello stesso oggetto.

---