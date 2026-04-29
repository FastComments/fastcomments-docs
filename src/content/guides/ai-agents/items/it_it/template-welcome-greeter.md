**Template ID:** `welcome_greeter`

Il Welcome Greeter risponde calorosamente ai commentatori alla prima esperienza. È il modello a più basso rischio (nessuno strumento distruttivo) e un buon primo agente da mettere in produzione.

### Prompt iniziale integrato

[inline-code-attrs-start title = 'Prompt iniziale del modello Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Trigger

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

Questo evento si attiva esattamente una sola volta per utente, quindi l'agente non può andare in loop. Vedi [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Strumenti consentiti

- [`write_comment`](#tools-overview)

Quello è l'unico strumento - l'agente letteralmente non può moderare, votare, bannare o inviare DM.

### Aggiunte consigliate prima di andare in produzione

- **Imposta il nome visualizzato** su qualcosa di invitante - "Community Bot", la mascotte del tuo sito o il nome del tuo brand. Il nome visualizzato è ciò che i lettori vedono associato alla risposta di benvenuto.
- **Seleziona "Include page title, subtitle, description, and meta tags"** in [Context Options](#context-options). Le risposte del greeter migliorano notevolmente quando può fare riferimento al reale contenuto della pagina.
- **Considera restrizioni di lingua/locale** se operi in più lingue. Una risposta di benvenuto nella lingua sbagliata è più stonata di una risposta mancata. Vedi [Scope: URL and Locale Filters](#scope-url-locale).

### Perché non sono necessarie approvazioni

L'agente scrive solo nuovi commenti e solo su un trigger one-shot. Nel peggiore dei casi: un saluto imbarazzante. Non c'è alcuna azione distruttiva da limitare. La maggior parte degli operatori fa girare questo modello senza alcuna approvazione una volta che l'esecuzione di prova risulta pulita.

---