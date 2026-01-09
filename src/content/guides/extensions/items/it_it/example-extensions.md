---
In FastComments scriviamo le nostre estensioni usando la stessa API. Puoi visualizzare il codice non minimizzato di queste estensioni ai seguenti endpoint:

#### Modalità Scura

L'estensione Modalità Scura viene caricata condizionatamente quando viene rilevata una pagina "dark". Questa estensione aggiunge semplicemente del CSS al widget dei commenti. In questo modo non dobbiamo caricare il CSS della modalità scura quando non è necessario.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderazione

Quando l'utente corrente è un moderatore, utilizziamo l'estensione di moderazione.

Questo è un buon esempio per aggiungere listener per eventi basati sul click, effettuare richieste API, e aggiungere elementi al menu dei commenti e alle aree dei commenti.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Chat dal vivo

L'estensione Chat dal vivo (in combinazione con altre configurazioni e stili) trasforma il widget dei commenti in un componente di chat dal vivo.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js

---