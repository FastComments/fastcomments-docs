#### "Registration token not found, expired, or already used"

Il token nell'URL di registrazione è valido per 30 minuti e può essere utilizzato una sola volta. Se il tuo LMS ha impiegato più tempo, o se la registrazione è stata riprovata dopo essere andata a buon fine, il token verrà rifiutato. Genera un URL nuovo nella pagina di Configurazione LTI 1.3 di FastComments e riprova.

#### "Platform rejected registration"

Il tuo LMS ha rifiutato la stretta di mano di registrazione. Le cause più comuni:

- **Tool already registered with the same client name.** Alcune piattaforme (in particolare D2L) rifiutano una seconda registrazione di "FastComments" finché la precedente non viene eliminata. Rimuovi il vecchio strumento nel tuo LMS, poi riprova.
- **Wrong field in the LMS.** Assicurati di aver incollato l'URL nel campo **endpoint di registrazione / endpoint di avvio registrazione dello strumento**, non nel campo launch URL o login URL.
- **The LMS doesn't actually support Dynamic Registration.** Versioni più vecchie di Moodle e Blackboard dichiarano LTI 1.3 ma permettono solo la configurazione manuale. Controlla la documentazione della tua piattaforma.

#### "Failed to fetch platform configuration"

FastComments non è riuscito a leggere il documento openid-configuration del tuo LMS. È raro e di solito significa che il LMS ha fornito un URL di discovery malformato o irraggiungibile. Contatta il supporto del tuo LMS.

#### Launch shows "Configuration not found"

O la configurazione in FastComments è stata eliminata, o il lancio è arrivato da una coppia `iss`/`client_id` che non riconosciamo. Se hai eliminato e re-registrato, istruisci il tuo LMS a rimuovere e riaggiungere lo strumento FastComments in modo che ottenga il nuovo `client_id`.

#### Launch shows "Deployment not registered"

Hai avviato FastComments da un deployment di Brightspace/Moodle/Blackboard diverso da quello in cui è stato lanciato la prima volta. FastComments fissa il `deployment_id` al primo avvio come controllo di sicurezza. Per aggiungere un nuovo deployment sotto lo stesso client, contatta il supporto — aggiungeremo l'ID di deployment alla configurazione.

#### Launch shows "Unsupported message_type"

Il LMS ha inviato un messaggio LTI che FastComments non gestisce (es. `LtiSubmissionReviewRequest`). FastComments supporta solamente il lancio standard resource-link e i flussi di deep-linking. Contattaci se ti serve un tipo di messaggio specifico aggiunto.

#### Iframe doesn't resize

La maggior parte degli LMS ridimensiona automaticamente gli iframe LTI. Se il tuo non lo fa, verifica che le impostazioni di avvio del LMS permettano allo strumento di inviare eventi postMessage al frame parent. FastComments emette sia messaggi di resize in stile Canvas (`lti.frameResize`) sia quelli secondo la specifica IMS (`org.imsglobal.lti.frameResize`).