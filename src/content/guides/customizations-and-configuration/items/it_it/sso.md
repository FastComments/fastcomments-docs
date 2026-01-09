SSO, o single-sign-on, è un insieme di convenzioni usate per permettere a te o ai tuoi utenti di usare FastComments senza dover creare un altro account.

Assumendo che non permetti commenti anonimi, è necessario un account per commentare con FastComments. Rendiamo questo processo di registrazione molto semplice: l'utente lascia semplicemente la sua email quando commenta.
Tuttavia, capiamo che anche questo è un'ulteriore frizione che alcuni siti vogliono evitare.

Possiamo ridurre questa frizione avendo un unico flusso di accesso per l'intero sito.

### Come lo ottengo?
Tutti i tipi di account attualmente hanno accesso a SSO. Tuttavia, il numero massimo di utenti SSO varierà a seconda del tuo pacchetto. Come per le altre funzionalità, i piani Pro e superiori forniscono supporto diretto allo sviluppo.

Confrontiamo le opzioni, e poi entriamo nei dettagli di ciascuna.

### Migrazione di utenti e commenti

Quando si esegue la migrazione da una piattaforma con SSO come Disqus, avrai già utenti e i loro commenti.

I commenti vengono importati come parte della tua migrazione, tramite l'API, la nostra Import UI, o il supporto clienti. L'Import UI è preferita se supporta la piattaforma da cui stai migrando, poiché incorpora gestione degli errori, estrazione e upload di avatar e media, e un sistema di monitoraggio dei job in batch.

Gli utenti stessi vengono aggiunti automaticamente quando visualizzano per la prima volta le discussioni dei commenti. In alternativa, possono essere aggiunti in anticipo tramite l'API, ma questo lavoro non offre molti vantaggi.

Se i commenti vengono importati e gli utenti SSO non vengono aggiunti manualmente tramite l'API, allora i commenti verranno automaticamente assegnati all'account dell'utente la prima volta che questo viene creato quando visualizza una qualsiasi discussione di commenti. Potranno quindi gestire, modificare ed eliminare i commenti che avevano scritto originariamente.

La migrazione automatica viene eseguita tramite email o username. Alcune piattaforme non forniscono le email nell'export, come Disqus, quindi in questo caso facciamo fallback sull'username.
- Finché passi un username corrispondente e una email nel payload SSO, aggiungeremo l'email ai singoli oggetti commento in modo che le notifiche e le menzioni funzionino.

Se desideri importare commenti e utenti contemporaneamente, lavora con il supporto per migrare i commenti agli account rispettivi degli utenti dopo che questi sono stati importati tramite l'API.

Quindi, per riassumere, il percorso più semplice da seguire per la migrazione è:

1. Importa i commenti.
   1. Avatar e altri media vengono migrati automaticamente se si utilizza l'Import UI in `Manage Data -> Imports`.
2. Configura Secure o Simple SSO.
3. Lascia che la migrazione avvenga per utente automaticamente quando eseguono il login per la prima volta.
   1. Questo di solito aggiunge meno di un secondo al tempo di caricamento della pagina se l'utente ha meno di 50k commenti.

### Utenti WordPress
Se stai usando il nostro <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">plugin per WordPress</a> allora non c'è codice da scrivere! Vai semplicemente alla pagina Admin del plugin, clicca su Impostazioni SSO, e poi su Abilita.

Questo ti porterà a un wizard a singolo click che creerà la tua API key, la invierà alla tua installazione WordPress e attiverà SSO. Abbiamo consolidato tutto questo in un singolo click per te.

Nota che se stai installando il plugin per la prima volta dovrai completare il processo di setup prima di vedere la pagina di amministrazione con il pulsante Impostazioni SSO.

#### SSO per WordPress - Moderatori

Nota che attualmente, affinché il badge "Moderatore" appaia accanto ai tuoi moderatori quando commentano con il plugin per WordPress di FastComments,
devono anche essere aggiunti come Moderatore nella dashboard di FastComments e avere la loro email verificata.

### Integrazioni personalizzate

Per integrazioni personalizzate, ci sono due opzioni.

### Opzione Uno - Secure SSO

Con Secure SSO, FastComments sa che l'utente che commenta, vota e legge i commenti è un utente reale del tuo sito.

Finché crei un payload valido, l'utente avrà sempre un'esperienza di commento senza interruzioni.

Con Secure SSO, il payload SSO viene creato **lato server** usando l'autenticazione HMAC e poi passato al widget sul **client**.

Con Secure SSO, l'account dell'utente è **completamente separato** dal resto della base utenti di FastComments. Questo significa che se abbiamo due partner
Azienda A e Azienda B, ognuno può avere un utente SSO con lo username "Bob".

#### Requisiti
- Conoscenze di base nello sviluppo backend.
- Conoscenze di base nella gestione di chiavi API segrete.
- Conoscenze di base nello sviluppo di API o nel server-side rendering.

#### Vantaggi
- Sicuro.
- Esperienza di commento senza interruzioni.

#### Svantaggi
- Richiede sviluppo backend.

#### Aggiornamento dei dati utente

Con Secure SSO, ogni volta che passi il payload utente sso, aggiorneremo il loro utente con le informazioni più recenti. Per esempio, se
l'utente ha uno username `X`, e passi `Y` nel payload SSO, il loro username diventerà `Y`.

Se vuoi rimuovere dei valori usando questo approccio allora impostali a `null` (non `undefined`).

#### API Secure SSO

Forniamo inoltre un'API per interagire con gli utenti SSO. Vedi [la documentazione](/guide-api.html#sso-user-structure).

Nota che quando si utilizza Secure SSO, gli utenti vengono creati automaticamente dietro le quinte al caricamento della pagina. Non è necessario importare in blocco i tuoi utenti.

### Opzione Due - Simple SSO

L'alternativa a Secure SSO è semplicemente passare le informazioni dell'utente al widget dei commenti.

Fornire un'email con Simple SSO non è obbligatorio, tuttavia senza questa i loro commenti appariranno come "Non verificato".

<sup>Nota!</sup> A partire dai primi mesi del 2022 i nomi utente con Simple SSO non devono essere unici su tutto FastComments.com.

Idealmente, Simple SSO dovrebbe essere scelto solo quando si sviluppa su una piattaforma che non fornisce accesso al backend.

#### Requisiti
- Conoscenze di base nello sviluppo lato client.
- È necessario conoscere almeno l'email dell'utente.

#### Vantaggi
- Semplice.
- Tutte le attività vengono comunque verificate.
- L'utente non inserisce mai il proprio username o email.

#### Svantaggi
- Meno sicuro rispetto a Secure SSO poiché il payload lato client potrebbe essere manipolato per impersonare qualsiasi utente.

#### API Simple SSO

Gli utenti creati automaticamente tramite il flusso Simple SSO vengono memorizzati come oggetti `SSOUser`. Possono essere accessi e gestiti tramite l'API `SSOUser`. Vedi [la documentazione](/guide-api.html#sso-user-structure).