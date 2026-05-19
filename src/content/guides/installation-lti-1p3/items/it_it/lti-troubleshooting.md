#### "Token di registrazione non trovato, scaduto o già utilizzato"

Il token nell'URL di registrazione (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">recuperalo qui</a>) è valido per 30 minuti e può essere usato una sola volta. Se il tuo LMS ha impiegato più tempo, o se la registrazione è stata ripetuta dopo un successo, il token verrà rifiutato. Genera un nuovo URL nella pagina di configurazione LTI 1.3 di FastComments e riprova.

#### "Platform rejected registration"

Il tuo LMS ha rifiutato l'handshake di registrazione. Le cause più comuni:

- **Tool already registered with the same client name.** Alcune piattaforme (in particolare D2L) rifiutano una seconda registrazione di "FastComments" finché quella precedente non viene eliminata. Rimuovi il vecchio tool nel tuo LMS, quindi riprova.
- **Wrong field in the LMS.** Assicurati di aver incollato l'URL nel campo **registration / tool initiation registration endpoint**, non nel campo launch URL o login URL.
- **The LMS doesn't actually support Dynamic Registration.** Versioni più vecchie di Moodle e Blackboard dichiarano il supporto per LTI 1.3 ma consentono solo la configurazione manuale. Controlla la documentazione della tua piattaforma.

#### "Failed to fetch platform configuration"

FastComments non è riuscito a leggere il documento openid-configuration del tuo LMS. È raro e di solito significa che il LMS ha fornito un URL di discovery malformato o non raggiungibile. Contatta il supporto del tuo LMS.

#### Launch shows "Configuration not found"

O la configurazione in FastComments è stata eliminata, oppure l'avvio proviene da una coppia `iss`/`client_id` che non riconosciamo. Se hai eliminato e poi registrato di nuovo, istruisci il tuo LMS a rimuovere e riaggiungere lo strumento FastComments in modo che ottenga il nuovo client_id.

#### Launch shows "Deployment not registered"

Hai avviato FastComments da una deployment di Brightspace/Moodle/Blackboard diversa da quella in cui è stato lanciato inizialmente. FastComments fissa il `deployment_id` al primo avvio come controllo di sicurezza. Per aggiungere una nuova deployment sotto lo stesso client, contatta il supporto - aggiungeremo il deployment ID alla configurazione.

#### Launch shows "Unsupported message_type"

Il LMS ha inviato un messaggio LTI che FastComments non gestisce (es. `LtiSubmissionReviewRequest`). FastComments supporta solo l'avvio standard resource-link e i flussi di deep-linking. Contattaci se hai bisogno che venga aggiunto un tipo di messaggio specifico.

#### Iframe doesn't resize

La maggior parte dei LMS ridimensiona automaticamente gli iframe LTI. Se il tuo non lo fa, verifica che le impostazioni di avvio del LMS permettano allo strumento di inviare eventi postMessage al frame padre. FastComments emette sia messaggi di ridimensionamento in stile Canvas (`lti.frameResize`) sia secondo la specifica IMS (`org.imsglobal.lti.frameResize`) di resize.