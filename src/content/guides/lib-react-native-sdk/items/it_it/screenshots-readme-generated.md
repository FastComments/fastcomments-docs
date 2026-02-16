#### Tema: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Predefinito
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Editor WYSIWYG nativo con supporto alle immagini!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Editor di testo ricco

Questa libreria utilizza l'editor 10tap per la funzionalità di editing di testo ricco, che fornisce un'esperienza WYSIWYG potente.

### Opzioni di configurazione

Questa libreria mira a supportare tutte le opzioni di configurazione definite in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), esattamente come l'implementazione web.

### Concetti di FastComments

I concetti principali da conoscere per iniziare sono `tenantId` e `urlId`. `tenantId` è l'identificativo del tuo account FastComments.com. `urlId` è l'entità a cui verranno legati i thread di commento. Questo può essere un URL di pagina, o un id prodotto, un id articolo, ecc.

### Notifiche utente

FastComments supporta le notifiche per [molteplici scenari](https://docs.fastcomments.com/guide-notifications.html). Le notifiche sono configurabili, è possibile rinunciare globalmente oppure a livello di singola notifica/commento, e supportano le iscrizioni a livello di pagina in modo che gli utenti possano iscriversi ai thread di una pagina o articolo specifico.

Ad esempio, è possibile usare Secure SSO per autenticare l'utente e poi interrogare periodicamente le notifiche non lette e inviarle all'utente.

Vedi [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) per sapere come ottenere e tradurre le notifiche utente non lette.

### Browser GIF

Per impostazione predefinita, non è abilitata la selezione di immagini o gif. Vedi [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) per sapere come supportare l'upload di immagini e gif. In questa libreria è fornito un Browser GIF che anonimizza le ricerche e le immagini fornite, basta usarlo.

### Prestazioni

Per favore apri un ticket con un esempio che riproduca il problema, includendo il dispositivo usato, se identifichi problemi di prestazioni. Le prestazioni sono di primaria importanza in tutte le librerie FastComments.