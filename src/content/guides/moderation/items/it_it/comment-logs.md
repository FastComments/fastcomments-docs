FastComments traccia automaticamente eventi dettagliati per ogni commento per fornire trasparenza nelle decisioni di moderazione e nelle azioni di sistema. Questi log ti aiutano a capire perché un commento è stato approvato, segnalato come spam o ha avuto il suo stato modificato.

## Accesso ai Log dei Commenti

Per visualizzare i log di un commento specifico:

1. Vai alla pagina **Moderate Comments** nella tua dashboard di FastComments  
2. Trova il commento che desideri ispezionare  
3. Clicca sul pulsante **View Logs** (icona a forma di orologio) nella barra delle azioni del commento  
4. Apparirà una finestra di dialogo che mostra la cronologia completa degli eventi per quel commento  

Ogni voce del log mostra:
- **When** – Il timestamp dell'evento  
- **Who** – L'utente o il sistema che ha generato l'evento (quando applicabile)  
- **What** – Il tipo di azione o evento  
- **Details** – Contesto aggiuntivo come valori prima/dopo, nomi dei motori o dati correlati  

## Eventi del Log dei Commenti

Ogni commento mantiene un registro degli eventi che si verificano durante il suo ciclo di vita. Di seguito sono elencati i tipi di eventi tracciati:

### Eventi di Anonimizzazione
- **Anonymized** – Il contenuto del commento è stato cancellato e l'utente contrassegnato come eliminato  
- **RestoredFromAnonymized** – Il commento è stato ripristinato dallo stato anonimizzato  

### Eventi di Approvazione
- **ApprovedDueToPastComment** – Commento approvato perché l'utente ha precedentemente approvato commenti (include riferimento al commento passato)  
- **ApprovedIsAdmin** – Commento approvato perché l'utente è un amministratore  
- **NotApprovedRequiresApproval** – Il commento richiede approvazione manuale  
- **NotApprovedLowTrustFactor** – Commento non approvato a causa di un basso fattore di fiducia dell'utente (include il valore del fattore di fiducia)  

### Eventi di Approvazione dei Commenti del Profilo
Questi eventi si applicano specificamente ai commenti sui profili utente:

- **ApprovedProfileAutoApproveAll** – Commento del profilo auto-approvato perché il proprietario del profilo ha abilitato l'auto-approvazione per tutti i commenti  
- **ApprovedProfileTrusted** – Commento del profilo approvato perché il commentatore è affidabile (include riferimento al commento che ha stabilito la fiducia)  
- **NotApprovedProfileManualApproveAll** – Commento del profilo richiede approvazione manuale perché il proprietario del profilo ha abilitato l'approvazione manuale  
- **NotApprovedProfileNotTrusted** – Commento del profilo non approvato perché il commentatore non è affidabile  
- **NotApprovedProfileNewUser** – Commento del profilo non approvato perché il commentatore è un nuovo utente  

### Eventi di Rilevamento Spam
- **IsSpam** – Commento segnalato come spam dal motore di rilevamento (include quale motore ha preso la decisione)  
- **IsSpamDueToBadWords** – Commento segnalato come spam a causa del filtro di volgarità  
- **IsSpamFromLLM** – Commento segnalato come spam dal motore AI/LLM (include nome del motore, risposta e conteggio dei token)  
- **IsSpamRepeatComment** – Commento segnalato come spam per essere ripetitivo (include quale motore l'ha rilevato)  
- **NotSpamIsOnlyImage** – Commento non segnalato come spam perché contiene solo immagini  
- **NotSpamIsOnlyReacts** – Commento non segnalato come spam perché contiene solo reazioni  
- **NotSpamNoLinkOrMention** – Commento non segnalato come spam perché non contiene link o menzioni sospette  
- **NotSpamPerfectTrustFactor** – Commento non segnalato come spam a causa dell'alta fiducia dell'utente  
- **NotSpamTooShort** – Commento non segnalato come spam perché è troppo corto per l'analisi  
- **NotSpamSkipped** – Controllo spam saltato  
- **NotSpamFromEngine** – Commento determinato non spam dal motore di rilevamento (include nome del motore e fattore di fiducia)  

### Eventi di Parole Offensivi/Profanità
- **BadWordsCheckFailed** – Il controllo del filtro di profanità ha riscontrato un errore  
- **BadWordsFoundBadPhrase** – Il filtro di profanità ha rilevato una frase inappropriata (include la frase)  
- **BadWordsFoundBadWord** – Il filtro di profanità ha rilevato una parola inappropriata (include la parola)  
- **BadWordsNoDefinitionForLocale** – Nessuna definizione di profanità disponibile per la lingua del commento (include la locale)  

### Eventi di Verifica Utente
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** – Il commento richiede verifica ma l'utente non è in una sessione verificata  
- **CommentMustBeVerifiedToApproveNotVerifiedYet** – Il commento richiede verifica ma l'utente non è ancora verificato  
- **InVerifiedSession** – L'utente che pubblica il commento è in una sessione verificata  
- **SentVerificationEmailNoSession** – Email di verifica inviata a un utente non verificato  
- **SentWelcomeEmail** – Email di benvenuto inviata a un nuovo utente  

### Eventi di Fiducia e Sicurezza
- **TrustFactorChanged** – Il fattore di fiducia dell'utente è stato modificato (include valori prima e dopo)  
- **SpamFilterDisabledBecauseAdmin** – Filtraggio spam bypassato per l'utente amministratore  
- **TenantSpamFilterDisabled** – Filtraggio spam disabilitato per l'intero tenant  
- **RepeatCommentCheckIgnored** – Controllo commenti ripetitivi bypassato (include il motivo)  
- **UserIsAdmin** – Utente identificato come amministratore  
- **UserIsAdminParentTenant** – Utente identificato come amministratore del tenant genitore  
- **UserIsAdminViaSSO** – Utente identificato come amministratore via SSO  
- **UserIsMod** – Utente identificato come moderatore  

### Cambiamenti di Stato del Commento
Gli eventi di cambiamento di stato includono valori prima e dopo, più l'utente che ha effettuato il cambiamento:

- **ExpireStatusChanged** – Lo stato di scadenza del commento è stato modificato  
- **ReviewStatusChanged** – Lo stato di revisione del commento è stato cambiato  
- **SpamStatusChanged** – Lo stato di spam del commento è stato aggiornato  
- **ApproveStatusChanged** – Lo stato di approvazione del commento è stato cambiato  
- **TextChanged** – Il contenuto testuale del commento è stato modificato (include testo prima e dopo)  
- **VotesChanged** – I conteggi dei voti del commento sono stati aggiornati (include dettagliata ripartizione dei voti)  
- **Flagged** – Il commento è stato segnalato dagli utenti  
- **UnFlagged** – Le segnalazioni del commento sono state rimosse  

### Azioni di Moderazione
- **Pinned** – Il commento è stato fissato dal moderatore (include chi lo ha fissato)  
- **UnPinned** – Il commento è stato sbloccato dal moderatore (include chi lo ha sbloccato)  

### Eventi di Notifica
- **CreatedNotifications** – Le notifiche sono state create per il commento (include il conteggio delle notifiche)  
- **NotificationCreateFailure** – Creazione delle notifiche fallita  
- **BadgeAwarded** – Il badge utente è stato assegnato per il commento (include il nome del badge)  

### Eventi di Notifica per Menzioni e Risposte
Questi eventi indicano la persona che riceverebbe l'email o la notifica. Quando non è stato inviato nulla, la colonna Dettagli indica il motivo.

- **MentionEmailSent** – Un utente menzionato nel commento ha ricevuto un'email  
- **MentionEmailSkipped** – Un utente menzionato non ha ricevuto l'email (include il motivo)  
- **MentionHeldForApproval** – L'email di menzione è in attesa fino a quando il commento non viene approvato  
- **MentionNotificationCreated** – Un utente menzionato ha ricevuto una notifica in-app  
- **MentionNotificationSkipped** – Un utente menzionato non ha ricevuto una notifica in-app (include il motivo)  
- **ReplyEmailSent** – L'autore del commento a cui si risponde è stato emailato riguardo a questa risposta  
- **ReplyEmailSkipped** – L'autore del commento a cui si risponde non è stato emailato (include il motivo)  
- **ReplyNotificationSkipped** – L'autore del commento a cui si risponde non ha ricevuto una notifica in-app (include il motivo)  

Motivi mostrati quando un'email o una notifica non è stata inviata:
- L'utente non esiste più, o non ha un indirizzo email  
- L'utente ha disattivato le notifiche email, o ha disattivato le notifiche per quel thread  
- Uno dei due utenti ha bloccato l'altro  
- Gli utenti non sono in nessuno dei gruppi SSO comuni  
- L'indirizzo email dell'utente è nella lista di soppressione dopo un rimbalzo o una segnalazione di spam (vedi [Email Suppression Management](/guide-notifications.html#email-suppression-management))  
- L'indirizzo email dell'utente è su example.com, che non può ricevere email  
- Il commento è stato contrassegnato come spam, eliminato, o non approvato entro 7 giorni  
- Il commento a cui si risponde è stato lasciato in forma anonima  
- L'utente ha risposto al proprio commento  
- L'utente è stato menzionato nella risposta, quindi ha ricevuto l'email di menzione invece dell'email di risposta  
- L'utente aveva già una notifica di risposta per il commento  
- Invio fallito 5 volte  

Se la consegna fallisce o raggiunge un limite di invio, l'email viene messa in coda per il ritentativo e la voce di log lo indica.  

### Eventi di Pubblicazione
- **PublishedLive** – Il commento è stato pubblicato ai sottoscrittori live (include il conteggio dei sottoscrittori)  

### Eventi di Integrazione
- **WebhookSynced** – Il commento è stato sincronizzato via webhook  

### Eventi di Regola Spam
- **SpamRuleMatch** – Il commento ha corrisposto a una regola spam personalizzata (include i dettagli della regola)  

### Eventi di Localizzazione
- **LocaleDetectedFromText** – Il locale della lingua è stato automaticamente rilevato dal testo del commento (include lingua e locale rilevati)  

## Casi d'Uso per i Log dei Commenti

I log dei commenti sono generati automaticamente e memorizzati con ogni commento. Forniscono preziose informazioni per:

- **Understanding moderation decisions** – Vedere esattamente perché un commento è stato approvato, trattenuto per revisione, o contrassegnato come spam  
- **Debugging approval/spam issues** – Tracciare la logica decisionale quando i commenti non si comportano come previsto  
- **Tracking user behavior patterns** – Monitorare i cambiamenti del fattore di fiducia e lo stato di verifica  
- **Auditing moderator actions** – Rivedere quali azioni i moderatori hanno effettuato su commenti specifici  
- **Investigating spam filter effectiveness** – Vedere quali motori di rilevamento stanno catturando lo spam e quali no  
- **Troubleshooting integrations** – Verificare la sincronizzazione dei webhook e la consegna delle notifiche  

Questi log aiutano a mantenere la trasparenza nel processo di moderazione e assistono nell'affinare il comportamento del tuo sistema di commenti.