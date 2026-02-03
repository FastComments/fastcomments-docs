FastComments tiene automaticamente traccia di eventi dettagliati per ogni commento per fornire trasparenza nelle decisioni di moderazione e nelle azioni del sistema. Questi log aiutano a capire perché un commento è stato approvato, segnalato come spam o ha avuto il suo stato modificato.

## Accesso ai log dei commenti

Per visualizzare i log di uno specifico commento:

1. Vai alla pagina **Moderate Comments** nella dashboard di FastComments
2. Trova il commento che vuoi ispezionare
3. Clicca il pulsante **View Logs** (icona dell'orologio) nella barra delle azioni del commento
4. Apparirà una finestra di dialogo che mostra la cronologia completa degli eventi per quel commento

Ogni voce del log mostra:
- **When** - Il timestamp dell'evento
- **Who** - L'utente o il sistema che ha attivato l'evento (quando applicabile)
- **What** - Il tipo di azione o evento
- **Details** - Contesto aggiuntivo come valori prima/dopo, nomi dei motori o dati correlati

## Eventi del log dei commenti

Ogni commento mantiene un log degli eventi che si verificano durante il suo ciclo di vita. Di seguito sono elencati i tipi di eventi che vengono tracciati:

### Anonymization Events
- **Anonymized** - Il contenuto del commento è stato cancellato e l'utente segnato come eliminato
- **RestoredFromAnonymized** - Il commento è stato ripristinato dallo stato anonimizzato

### Approval Events
- **ApprovedDueToPastComment** - Commento approvato perché l'utente ha commenti precedenti approvati (include riferimento al commento passato)
- **ApprovedIsAdmin** - Commento approvato perché l'utente è un amministratore
- **NotApprovedRequiresApproval** - Il commento richiede approvazione manuale
- **NotApprovedLowTrustFactor** - Commento non approvato a causa di un basso fattore di fiducia dell'utente (include il valore del trust factor)

### Profile Comment Approval Events

Questi eventi si applicano specificamente ai commenti nei profili utente:

- **ApprovedProfileAutoApproveAll** - Commento sul profilo auto-approvato perché il proprietario del profilo ha abilitato l'auto-approvazione per tutti i commenti
- **ApprovedProfileTrusted** - Commento sul profilo approvato perché il commentatore è considerato affidabile (include riferimento al commento che ha stabilito la fiducia)
- **NotApprovedProfileManualApproveAll** - Commento sul profilo che richiede approvazione manuale perché il proprietario del profilo ha abilitato l'approvazione manuale
- **NotApprovedProfileNotTrusted** - Commento sul profilo non approvato perché il commentatore non è considerato affidabile
- **NotApprovedProfileNewUser** - Commento sul profilo non approvato perché il commentatore è un utente nuovo

### Spam Detection Events
- **IsSpam** - Commento segnalato come spam dal motore di rilevamento (include quale motore ha preso la decisione)
- **IsSpamDueToBadWords** - Commento segnalato come spam a causa del filtro di volgarità
- **IsSpamFromLLM** - Commento segnalato come spam dal motore AI/LLM (include il nome del motore, la risposta e il conteggio dei token)
- **IsSpamRepeatComment** - Commento segnalato come spam per essere ripetitivo (include quale motore lo ha rilevato)
- **NotSpamIsOnlyImage** - Commento non segnalato come spam perché contiene solo immagini
- **NotSpamIsOnlyReacts** - Commento non segnalato come spam perché contiene solo reazioni
- **NotSpamNoLinkOrMention** - Commento non segnalato come spam a causa dell'assenza di link o mention sospetti
- **NotSpamPerfectTrustFactor** - Commento non segnalato come spam a causa di un alto trust factor dell'utente
- **NotSpamTooShort** - Commento non segnalato come spam perché troppo breve per essere analizzato
- **NotSpamSkipped** - Il controllo anti-spam è stato saltato
- **NotSpamFromEngine** - Commento determinato non spam dal motore di rilevamento (include nome del motore e trust factor)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Il controllo del filtro di volgarità ha riscontrato un errore
- **BadWordsFoundBadPhrase** - Il filtro di volgarità ha rilevato una frase inappropriata (include la frase)
- **BadWordsFoundBadWord** - Il filtro di volgarità ha rilevato una parola inappropriata (include la parola)
- **BadWordsNoDefinitionForLocale** - Nessuna definizione di volgarità disponibile per la lingua del commento (include la locale)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Il commento richiede verifica ma l'utente non è in una sessione verificata
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Il commento richiede verifica ma l'utente non è ancora verificato
- **InVerifiedSession** - L'utente che pubblica il commento è in una sessione verificata
- **SentVerificationEmailNoSession** - Email di verifica inviata a un utente non verificato
- **SentWelcomeEmail** - Email di benvenuto inviata al nuovo utente

### Trust and Security Events
- **TrustFactorChanged** - Il trust factor dell'utente è stato modificato (include valori prima e dopo)
- **SpamFilterDisabledBecauseAdmin** - Il filtraggio dello spam è stato bypassato per un utente admin
- **TenantSpamFilterDisabled** - Il filtraggio dello spam disabilitato per l'intero tenant
- **RepeatCommentCheckIgnored** - Il controllo dei commenti ripetuti è stato ignorato (include il motivo)
- **UserIsAdmin** - Utente identificato come amministratore
- **UserIsAdminParentTenant** - Utente identificato come amministratore del tenant principale
- **UserIsAdminViaSSO** - Utente identificato come amministratore tramite SSO
- **UserIsMod** - Utente identificato come moderatore

### Comment Status Changes

Gli eventi di cambio stato includono i valori prima e dopo, oltre all'utente che ha effettuato la modifica:

- **ExpireStatusChanged** - Lo stato di scadenza del commento è stato modificato
- **ReviewStatusChanged** - Lo stato di revisione del commento è stato modificato
- **SpamStatusChanged** - Lo stato di spam del commento è stato aggiornato
- **ApproveStatusChanged** - Lo stato di approvazione del commento è stato modificato
- **TextChanged** - Il testo del commento è stato modificato (include testo prima e dopo)
- **VotesChanged** - I conteggi dei voti del commento sono stati aggiornati (include dettagliata suddivisione dei voti)
- **Flagged** - Il commento è stato segnalato dagli utenti
- **UnFlagged** - Le segnalazioni del commento sono state rimosse

### Moderation Actions
- **Pinned** - Il commento è stato fissato da un moderatore (include chi lo ha fissato)
- **UnPinned** - Il commento è stato rimosso dalla fissazione da un moderatore (include chi lo ha rimosso)

### Notification Events
- **CreatedNotifications** - Sono state create notifiche per il commento (include il conteggio delle notifiche)
- **NotificationCreateFailure** - Creazione delle notifiche fallita
- **BadgeAwarded** - All'utente è stato assegnato un badge per il commento (include il nome del badge)

### Publishing Events
- **PublishedLive** - Il commento è stato pubblicato ai sottoscrittori live (include il conteggio dei sottoscrittori)

### Integration Events
- **WebhookSynced** - Il commento è stato sincronizzato tramite webhook

### Spam Rule Events
- **SpamRuleMatch** - Il commento ha corrisposto a una regola spam personalizzata (include i dettagli della regola)

### Localization Events
- **LocaleDetectedFromText** - La lingua/locale è stata rilevata automaticamente dal testo del commento (include la lingua e la locale rilevate)

## Casi d'uso dei log dei commenti

I log dei commenti vengono generati automaticamente e memorizzati con ogni commento. Forniscono preziose informazioni per:

- **Comprendere le decisioni di moderazione** - Vedere esattamente perché un commento è stato approvato, messo in revisione o contrassegnato come spam
- **Debug di problemi di approvazione/spam** - Tracciare la logica decisionale quando i commenti non si comportano come previsto
- **Monitorare i modelli di comportamento degli utenti** - Monitorare le variazioni del trust factor e lo stato di verifica
- **Audit delle azioni dei moderatori** - Revisionare quali azioni i moderatori hanno eseguito su commenti specifici
- **Indagare sull'efficacia del filtro antispam** - Vedere quali motori di rilevamento individuano lo spam e quali no
- **Risoluzione dei problemi di integrazione** - Verificare le sincronizzazioni webhook e la consegna delle notifiche

Questi log aiutano a mantenere la trasparenza nel processo di moderazione e assistono nell'ottimizzazione del comportamento del sistema di commenti.