---
**ID del template:** `welcome_greeter`

Il Welcome Greeter risponde calorosamente agli utenti che commentano per la prima volta. È il template a rischio più basso (nessuno strumento distruttivo) ed è un buon primo agente da mettere in produzione.

### Eventi

- **Un nuovo utente pubblica il suo primo commento su questo sito** (`NEW_USER_FIRST_COMMENT`).

Questo evento viene attivato esattamente una volta per utente, quindi l'agente non può andare in loop. Vedi [Trigger: Primo commento del nuovo utente](#trigger-new-user-first-comment).

### Strumenti consentiti

- [`write_comment`](#tools-overview)

Questo è l'unico strumento: l'agente letteralmente non può moderare, votare, bannare o inviare messaggi diretti (DM).

### Aggiunte consigliate prima di andare live

- **Imposta il Display name** su qualcosa di invitante - "Community Bot", la mascotte del tuo sito, o il nome del tuo brand. Il display name è ciò che i lettori vedono associato alla risposta di benvenuto.
- **Seleziona "Include page title, subtitle, description, and meta tags"** in [Context Options](#context-options). Le risposte del greeter migliorano notevolmente quando può riferirsi a ciò di cui parla effettivamente la pagina.
- **Valuta restrizioni di localizzazione** se operi in più lingue. Una risposta di benvenuto nella lingua sbagliata è più sconcertante di una risposta mancata. Vedi [Scope: URL and Locale Filters](#scope-url-locale).

### Perché non sono necessarie approvazioni

L'agente scrive solo nuovi commenti e solo su un trigger one-shot. Nel peggiore dei casi: un saluto imbarazzante. Non ci sono azioni distruttive da limitare. La maggior parte degli operatori esegue questo agente senza alcuna approvazione una volta che la prova a secco (dry-run) risulta pulita.

---