Per impostazione predefinita, FastComments include un rilevamento dello spam addestrabile.

Man mano che moderi i commenti e li contrassegni come **Spam**, oppure contrassegni come **Not Spam** i commenti individuati automaticamente come **Spam**, il sistema di rilevamento dello spam imparerà da queste azioni per determinare con maggiore precisione ciò che vuoi venga considerato spam.

I commenti contrassegnati come **Spam** non verranno approvati automaticamente, quindi non saranno visibili fino a quando non vengono esplicitamente contrassegnati come **Not Spam**.

Il rilevamento dello spam può essere disabilitato dalla pagina delle impostazioni di moderazione dei commenti.

### Diversi rilevatori di spam

FastComments supporta tre metodi per rilevare lo spam:

1. Un tradizionale classificatore Naïve-Bayes che viene continuamente addestrato, condiviso tra tutti i tenant di FastComments.com.
2. Un tradizionale classificatore Naïve-Bayes che viene continuamente addestrato, **isolato** al tuo tenant.
3. Utilizzo di ChatGPT 4.

Tutti hanno accesso sia al classificatore Naïve-Bayes condiviso sia a quello isolato.

L'opzione ChatGPT 4 è selezionabile nella pagina delle impostazioni di moderazione dei commenti se sei in regime di fatturazione Flex, poiché la fatturazione si basa sui token utilizzati.

### Fattore di fiducia

FastComments regola il filtro antispam per un utente in base al livello di fiducia che ha per il sito in questione.

Ad esempio, se gli amministratori hanno appuntato molti dei loro commenti, allora probabilmente si tratta di un utente molto affidabile. Oppure, se sono membri del sito da molto tempo e hanno molti commenti, il loro fattore di fiducia può essere elevato.

### SSO

I commenti pubblicati dagli utenti SSO possono essere considerati spam e saranno controllati come tali. L'eccezione è se l'utente SSO ha la stessa email di un utente del tenant che possiede una o più delle seguenti autorizzazioni:

- Account Owner
- Super Admin
- Comment Moderator Admin

Gli utenti SSO con queste autorizzazioni non avranno i loro commenti controllati per lo spam.

### Messaggi ripetuti

FastComments rileva e impedisce messaggi ripetuti. Rileva anche messaggi ripetuti molto simili per contribuire a prevenire lo spam. Questo non può essere disabilitato poiché impedisce che la nostra piattaforma venga usata per abusi. Se hai un alto fattore di fiducia, questo viene preso in considerazione durante la prevenzione dei messaggi ripetuti.