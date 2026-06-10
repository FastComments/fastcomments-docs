#### Skin: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skin: Default
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Editor WYSIWYG nativo con supporto alle immagini!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Editor di testo ricco

Questa libreria utilizza [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) per l'editing di testo ricco, che fornisce un'esperienza WYSIWYG potente. Lo stesso editor alimenta iOS, Android e il web (tramite `react-native-web`), quindi il compositore si comporta in modo coerente su tutte le piattaforme con un'unica implementazione.

`react-native-enriched` richiede la React Native New Architecture (Fabric) su native, e un bundler che risolva le condizioni di `exports` dei pacchetti (Metro con package exports / RN 0.72+). Il supporto web è attualmente sperimentale.

### Opzioni di configurazione

Questa libreria mira a supportare tutte le opzioni di configurazione definite in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), proprio come l'implementazione web.

### Concetti FastComments

I concetti principali di cui tenere conto per iniziare sono `tenantId` e `urlId`. `tenantId` è l'identificativo del tuo account FastComments.com. `urlId` è l'elemento a cui verranno legati i thread dei commenti. Questo potrebbe essere un URL di pagina, oppure un id prodotto, un id articolo, ecc.

### Notifiche utente

FastComments supporta le notifiche per [molti scenari](https://docs.fastcomments.com/guide-notifications.html). Le notifiche sono configurabili, è possibile disattivarle globalmente o a livello di notifica/commento, e supportano le sottoscrizioni a livello di pagina in modo che gli utenti possano iscriversi ai thread di una pagina o articolo specifico.

Ad esempio, è possibile utilizzare Secure SSO per autenticare l'utente e poi eseguire periodicamente polling per le notifiche non lette e inoltrarle all'utente.

Vedi [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) per come ottenere e tradurre le notifiche utente non lette.

### Browser GIF

Per impostazione predefinita, non è abilitata alcuna selezione di immagini o gif. Vedi [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) per come supportare caricamenti di immagini e gif. Questa libreria include un Browser GIF che anonimizza le ricerche e le immagini fornite, devi solo utilizzarlo.

### Prestazioni

Apri un ticket con un esempio riproducibile, incluso il dispositivo utilizzato, se individui problemi di prestazioni. Le prestazioni sono una priorità di prima classe in tutte le librerie FastComments.