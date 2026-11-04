#### "Token di registrazione non trovato, scaduto o già utilizzato"

Il token nel tuo URL di registrazione (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottenilo qui</a>) è valido per 30 minuti e può essere utilizzato una sola volta. Se la tua LMS ha impiegato più tempo, o se la registrazione è stata ritentata dopo aver avuto successo, il token verrà rifiutato. Genera un nuovo URL nella pagina di configurazione LTI 1.3 di FastComments e ricomincia da capo.

#### "Registrazione rifiutata dalla piattaforma"

La tua LMS ha rifiutato l'handshake di registrazione. Le cause più comuni:

- **Strumento già registrato con lo stesso nome client.** Alcune piattaforme (in particolare D2L) rifiutano una seconda registrazione di "FastComments" finché quella precedente non viene eliminata. Rimuovi il vecchio strumento nella tua LMS, poi riprova.
- **Campo errato nella LMS.** Assicurati di aver incollato l'URL nel campo **registration / tool initiation registration endpoint**, non nel campo launch URL o login URL.
- **La LMS in realtà non supporta la Dynamic Registration.** Le versioni più vecchie di Moodle e Blackboard dichiarano di supportare LTI 1.3 ma consentono solo la configurazione manuale. Consulta la documentazione della tua piattaforma.

#### "Impossibile recuperare la configurazione della piattaforma"

FastComments non è riuscito a leggere il documento openid-configuration della tua LMS. Questo è raro e di solito significa che la LMS ha fornito un URL di discovery malformato o non raggiungibile. Contatta il supporto della tua LMS.

#### Il lancio mostra "Configuration not found"

Oppure la configurazione in FastComments è stata eliminata, oppure il lancio proviene da una coppia `iss`/`client_id` che non riconosciamo. Se hai eliminato e registrato di nuovo, istruisci la tua LMS a rimuovere e riaggiungere lo strumento FastComments così che ottenga il nuovo client_id.

#### Il lancio mostra "Deployment not registered"

Hai lanciato FastComments da un deployment Brightspace/Moodle/Blackboard diverso da quello in cui è stato lanciato per la prima volta. FastComments fissa il `deployment_id` al primo lancio come controllo di sicurezza. Per aggiungere un nuovo deployment sotto lo stesso client, contatta il supporto: aggiungeremo l'ID del deployment alla configurazione.

#### Il lancio mostra "Unsupported message_type"

La LMS ha inviato un messaggio LTI che FastComments non gestisce (ad es. `LtiSubmissionReviewRequest`). FastComments supporta solo il lancio standard resource-link e i flussi di deep-linking. Contattaci se ti serve l'aggiunta di un tipo di messaggio specifico.

#### L'iframe non si ridimensiona

La maggior parte delle LMS ridimensiona automaticamente gli iframe LTI. Se la tua non lo fa, verifica che le impostazioni di lancio della LMS consentano allo strumento di inviare eventi postMessage al frame padre. FastComments emette messaggi di ridimensionamento sia in stile Canvas (`lti.frameResize`) sia secondo le specifiche IMS (`org.imsglobal.lti.frameResize`).