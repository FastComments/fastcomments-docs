FastComments funziona con siti riservati ai membri utilizzando ciò che viene chiamato SSO, ovvero single-sign-on. I tuoi membri effettuano l'accesso al tuo sito WordPress, ma
non devono preoccuparsi di creare un account con FastComments, o di accedere tramite social media, per commentare. Se i tuoi membri (inclusi gli amministratori) sono connessi al
tuo sito WordPress, potranno commentare. I tuoi amministratori e moderatori potranno moderare i commenti direttamente dai post del tuo blog WordPress, anche.

<sup>(Opzionale)</sup> Ricorda anche di aggiungere i tuoi amministratori a [Utenti e Amministratori](https://fastcomments.com/auth/my-account/users) e i moderatori a [Moderatori dei Commenti](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
per migliorare la loro esperienza e abilitare il tracciamento delle statistiche per i moderatori.

Lo SSO può essere abilitato andando alla dashboard del plugin e cliccando "Impostazioni SSO".

Per prima cosa dovrai abilitare la funzione "Chiunque può registrarsi" del tuo sito.

Tutte le informazioni degli utenti vengono trasferite in modo sicuro e trasparente a FastComments ogni volta che un utente visualizza un thread di commenti tramite [HMAC](https://en.wikipedia.org/wiki/HMAC).

Non è necessario eseguire una sincronizzazione iniziale o continua per copiare i tuoi membri sui server di FastComments. Questo avviene automaticamente quando visualizzano le discussioni dei commenti aprendo un post del blog.

## Nomi e Avatar

Il plugin aggiornerà automaticamente il nome visualizzato e l'avatar dell'utente in tutti i suoi commenti all'interno di FastComments quando visualizzeranno
qualsiasi thread di commenti. Gli avatar sono supportati tramite Gravatar o qualsiasi plugin di gestione avatar in WordPress poiché il plugin chiamerà `get_avatar_url`.