FastComments traccia automaticamente eventi dettagliati per ogni commento per fornire trasparenza nelle decisioni di moderazione e nelle azioni del sistema. Questi log aiutano a capire perché un commento è stato approvato, segnalato come spam o ha avuto il suo stato modificato.

Puoi visualizzare i log dei commenti per commenti individuali nella dashboard Moderate Comments selezionando un commento specifico.

## Comment Log Events

Ogni commento mantiene un log degli eventi che si verificano durante il suo ciclo di vita. Di seguito sono riportati i tipi di eventi che vengono tracciati:

### Anonymization Events
- **Anonymized** - Il contenuto del commento è stato cancellato e l'utente contrassegnato come eliminato

### Approval Events
- **ApprovedDueToPastComment** - Commento approvato perché l'utente ha precedenti commenti approvati
- **ApprovedIsAdmin** - Commento approvato perché l'utente è un amministratore
- **NotApprovedRequiresApproval** - Il commento richiede approvazione manuale

### Spam Detection Events
- **IsSpam** - Commento segnalato come spam dal motore di rilevamento
- **IsSpamDueToBadWords** - Commento segnalato come spam a causa del filtro di volgarità
- **IsSpamFromLLM** - Commento segnalato come spam dall'engine AI/LLM
- **IsSpamRepeatComment** - Commento segnalato come spam per ripetitività
- **NotSpamIsOnlyImage** - Commento non segnalato come spam perché contiene solo immagini
- **NotSpamIsOnlyReacts** - Commento non segnalato come spam perché contiene solo reazioni
- **NotSpamNoLinkOrMention** - Commento non segnalato come spam perché non contiene link o menzioni sospette
- **NotSpamPerfectTrustFactor** - Commento non segnalato come spam a causa dell'elevato livello di fiducia dell'utente
- **NotSpamTooShort** - Commento non segnalato come spam perché troppo breve per essere analizzato
- **NotSpamSkipped** - Il controllo antispam è stato saltato
- **NotSpamFromEngine** - Commento determinato non spam dal motore di rilevamento

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Il controllo del filtro di volgarità ha riscontrato un errore
- **BadWordsFoundBadPhrase** - Il filtro di volgarità ha rilevato una frase inappropriata
- **BadWordsFoundBadWord** - Il filtro di volgarità ha rilevato una parola inappropriata
- **BadWordsNoDefinitionForLocale** - Nessuna definizione di volgarità disponibile per la lingua del commento

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Il commento richiede verifica ma l'utente non è in una sessione verificata
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Il commento richiede verifica ma l'utente non è ancora verificato
- **InVerifiedSession** - L'utente che pubblica il commento è in una sessione verificata
- **SentVerificationEmailNoSession** - Email di verifica inviata a un utente non verificato
- **SentWelcomeEmail** - Email di benvenuto inviata al nuovo utente

### Trust and Security Events
- **TrustFactorChanged** - Il fattore di fiducia dell'utente è stato modificato
- **SpamFilterDisabledBecauseAdmin** - Il filtro antispam è stato bypassato per un utente amministratore
- **TenantSpamFilterDisabled** - Il filtro antispam è stato disabilitato per l'intero tenant
- **RepeatCommentCheckIgnored** - Il controllo sui commenti ripetuti è stato ignorato
- **UserIsAdmin** - L'utente è stato identificato come amministratore
- **UserIsAdminParentTenant** - L'utente è stato identificato come amministratore del tenant principale
- **UserIsAdminViaSSO** - L'utente è stato identificato come amministratore tramite SSO
- **UserIsMod** - L'utente è stato identificato come moderatore

### Comment Status Changes
- **ExpireStatusChanged** - Lo stato di scadenza del commento è stato modificato
- **ReviewStatusChanged** - Lo stato di revisione del commento è stato modificato
- **SpamStatusChanged** - Lo stato di spam del commento è stato aggiornato
- **ApproveStatusChanged** - Lo stato di approvazione del commento è stato modificato
- **TextChanged** - Il contenuto testuale del commento è stato modificato
- **VotesChanged** - I conteggi dei voti del commento sono stati aggiornati
- **Flagged** - Il commento è stato segnalato dagli utenti
- **UnFlagged** - Le segnalazioni del commento sono state rimosse

### Moderation Actions
- **Pinned** - Il commento è stato fissato dal moderatore
- **UnPinned** - Il commento è stato rimosso dalla posizione fissata dal moderatore
- **RestoredFromAnonymized** - Il commento è stato ripristinato dallo stato anonimizzato

### Notification Events
- **CreatedNotifications** - Sono state create notifiche per il commento
- **NotificationCreateFailure** - Creazione delle notifiche fallita
- **BadgeAwarded** - All'utente è stato assegnato un badge per il commento

### Publishing Events
- **PublishedLive** - Il commento è stato pubblicato agli iscritti live

### Integration Events
- **WebhookSynced** - Il commento è stato sincronizzato tramite webhook

### Spam Rule Events
- **SpamRuleMatch** - Il commento ha corrisposto a una regola antispam personalizzata

## Accessing Comment Logs

I log dei commenti vengono generati automaticamente e archiviati con ogni commento. Forniscono informazioni preziose per:

- Comprendere le decisioni di moderazione
- Risolvere problemi relativi ad approvazione/spam
- Tracciare i modelli di comportamento degli utenti
- Audit delle azioni del sistema

Questi log aiutano a mantenere la trasparenza nel processo di moderazione e contribuiscono a ottimizzare il comportamento del sistema di commenti.