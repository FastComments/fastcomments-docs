#### Tema: Erebus
![Tema: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Tema: Predefinito
![Tema: Predefinito](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Editor WYSIWYG nativo con supporto alle immagini!
![Editor WYSIWYG nativo con supporto alle immagini](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Editor di testo ricco

Questa libreria utilizza l'editor 10tap per le funzionalità di editing rich text, che fornisce un'esperienza di modifica WYSIWYG potente.

### Opzioni di configurazione

Questa libreria mira a supportare tutte le opzioni di configurazione definite in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), proprio come l'implementazione web.

### Concetti di FastComments

I concetti principali da conoscere per iniziare sono `tenantId` e `urlId`. `tenantId` è l'identificatore del tuo account FastComments.com. `urlId` è l'entità a cui saranno legati i thread dei commenti. Questo potrebbe essere un URL di pagina, un id prodotto, un id articolo, ecc.

### Notifiche utente

FastComments supporta le notifiche per [molteplici scenari](https://docs.fastcomments.com/guide-notifications.html). Le notifiche sono configurabili, è possibile annullare l'iscrizione a livello globale o a livello di singola notifica/commento, e viene supportata la sottoscrizione a livello di pagina in modo che gli utenti possano iscriversi ai thread di una pagina o articolo specifico.

Ad esempio, è possibile utilizzare Secure SSO per autenticare l'utente e poi interrogare periodicamente le notifiche non lette e inviarle all'utente.

Vedi [l'esempio AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) per informazioni su come ottenere e tradurre le notifiche utente non lette.

### Browser GIF

Per impostazione predefinita non è abilitata la selezione di immagini o gif. Vedi [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) per come supportare l'upload di immagini e gif. Esiste un Gif Browser che anonimizza le ricerche e le immagini fornite in questa libreria, devi solo usarlo.

### Prestazioni

Apri un ticket con un esempio riproducibile, includendo il dispositivo utilizzato, se identifichi problemi di prestazioni. Le prestazioni sono una priorità in tutte le librerie FastComments.