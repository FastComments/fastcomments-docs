FastComments si integra con il sistema utenti di Drupal tramite SSO, o single sign-on. I tuoi utenti effettuano l'accesso al sito Drupal, e il modulo trasmette automaticamente la loro identità a FastComments. Nessun account aggiuntivo da creare, nessuna sincronizzazione iniziale da eseguire.

Il modulo supporta tre modalità SSO, configurabili in `Administration > Configuration > Content > FastComments`.

### Nessuno

Nessuno SSO. Gli utenti commentano come ospiti o creano un account FastComments. Usalo se il tuo sito è pubblico e non è necessario collegare i commenti agli utenti Drupal.

### Semplice

Trasmette il nome, l'email e l'avatar dell'utente Drupal a FastComments senza verifica lato server. Non è necessario un API Secret. Indicato per siti interni o a basso rischio.

### Sicuro (consigliato)

Utilizza [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) per verificare ogni identità utente con FastComments. Questa è la modalità che vuoi quando hai configurato un API Secret, ed è l'unica modalità che impedisce a un visitatore di impersonare un altro utente.

L'identità dell'utente viene trasmessa a FastComments ogni volta che un utente visualizza un thread di commenti. Non è necessaria alcuna sincronizzazione iniziale o continua che debba essere eseguita.

<sup>(Opzionale)</sup> Aggiungi i tuoi amministratori a [Utenti e amministratori](https://fastcomments.com/auth/my-account/users) e i moderatori a [Moderatori dei commenti](https://fastcomments.com/auth/my-account/moderate-comments/moderators) per migliorare la loro esperienza e abilitare il tracciamento delle statistiche per i moderatori.

Per un'analisi più approfondita di come funziona lo SSO, vedi la [sezione SSO](/guide-customizations-and-configuration.html#sso) della documentazione sulle personalizzazioni e la configurazione.