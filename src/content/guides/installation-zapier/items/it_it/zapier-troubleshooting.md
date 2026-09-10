## Risoluzione dei problemi

**"Non hai l'autorizzazione" durante la connessione.** L'utente connesso non è un amministratore API sull'account.  
Chiedi al proprietario dell'account di concedere l'autorizzazione API nella pagina Utenti, oppure connettiti come proprietario.

**La connessione è etichettata con il sito sbagliato.** La pagina di consenso collega l'account con cui eri connesso al momento. Disconnetti in Zapier, cambia account nella dashboard di FastComments e riconnetti.

**Gli eventi hanno smesso di arrivare.** Controlla la pagina Webhooks nella dashboard. Una sottoscrizione il cui endpoint ha continuato a fallire per sei giorni viene disabilitata automaticamente e mostra il motivo. Riabilitala lì, oppure disattiva e riattiva lo Zap. Se la sottoscrizione manca del tutto, qualcuno l'ha eliminata; disattivare e riattivare lo Zap la ricrea.

**Zapier dice che l'account deve essere riconnesso.** La connessione è stata revocata dalla pagina App connesse, l'utente che l'ha approvata ha perso l'autorizzazione API, o l'account è stato eliminato. Riconnetti da Zapier.

**Un'azione fallisce con "non ha accesso in scrittura".** La connessione è stata approvata con autorizzazione solo lettura. Riconnetti e approva entrambe le autorizzazioni.

**Limiti di velocità e crediti.** Azioni e ricerche consumano crediti API dal tuo piano e sono soggetti agli stessi limiti di velocità dell'API REST. I trigger non ne consumano. Uno Zap che raggiunge un limite viene ritentato da Zapier dopo il ritardo segnalato da FastComments.

**Il menu a discesa Dominio è vuoto.** I domini appaiono una volta configurati nella pagina Domini della dashboard di FastComments. Lascia il campo vuoto per ricevere eventi per tutti i domini.