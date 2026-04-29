Quando l'agent mette in coda un'approvazione, la piattaforma notifica i revisori via email. Due impostazioni nel modulo di modifica controllano questo: **chi** viene notificato e **con quale frequenza**.

### Chi: modalità di notifica

Due modalità:

- **Tutti gli amministratori e i moderatori** (predefinito) - ogni account owner, super admin e comment moderator admin sul tenant è un potenziale revisore.
- **Utenti specifici** - seleziona manualmente una lista tramite un dual-list picker nel modulo di modifica.

In entrambi i casi, un potenziale revisore deve avere un account sul tenant e un indirizzo email valido per ricevere le notifiche.

### Con quale frequenza: frequenza per utente

La **propria profilazione** di ciascun potenziale revisore determina la frequenza personale delle notifiche per le approvazioni dell'agent:

- **Immediato** (predefinito) - una email per ogni approvazione in sospeso, inviata non appena l'approvazione viene creata.
- **Ogni ora** - una email di riepilogo ogni ora che riassume tutte le approvazioni messe in coda in quell'ora.
- **Giornaliero** - una email di riepilogo ogni 24 ore.
- **Disabilitato** - nessuna email. L'utente può comunque revisionare le approvazioni tramite l'inbox UI; semplicemente non viene avvisato.

L'utente modifica questa impostazione nel proprio profilo, non nel modulo di modifica dell'agent. Questo è intenzionale: un tenant potrebbe avere dieci agent, e un moderatore non dovrebbe dover impostare la frequenza preferita su ogni agent singolarmente.

### Job cron che generano i digest

- **`hourly-agent-approval-digest`** - esegue una scansione ogni ora, raggruppa le approvazioni messe in coda dall'ultimo digest di ciascun utente, invia una email per utente.
- **`daily-agent-approval-digest`** - idem, giornalmente.
- **`agent-approval-reaper`** - elimina le approvazioni che hanno superato i 90 giorni indipendentemente dallo stato.

Gli hourly e daily digest cron sono limitati per destinatario: un utente con frequenza hourly viene processato dall'hourly cron e saltato dal daily (e viceversa). Gli utenti con frequenza Immediata vengono notificati tramite il percorso di codice approval-create, non dai cron.

### Stato di deduplicazione

La piattaforma traccia quali utenti hanno già ricevuto un'email per ciascuna approvazione. Una volta che un utente è stato notificato (immediatamente o in un digest), non riceverà di nuovo un'email per la stessa approvazione - anche se modifica la propria frequenza da Immediata a Giornaliera nel mezzo del ciclo.

### Approvare dall'email

Ogni email di notifica contiene un link di accesso firmato con un clic che porta il revisore direttamente alla pagina dei dettagli dell'approvazione, già autenticato. Da lì può approvare, rifiutare o aprire il flusso di [Affinamento dei prompt](#refining-prompts).

### Cosa succede se non esistono amministratori

Se `notifyMode` è `All admins and moderators` ma il tenant non ha super admins, comment moderator admins o account owners con email valide, la piattaforma registra un avviso e l'approvazione viene comunque messa in coda - semplicemente nessuno viene notificato. Rimarrà nella inbox finché qualcuno non la controllerà.

Se `notifyMode` è `Specific users` ma non hai selezionato alcun utente, stesso risultato.

### Cosa succede se le notifiche di billing sono disabilitate

[Avvisi sul budget](#budget-alerts) - le email relative al budget - vengono inviate ai billing admins **indipendentemente dalla preferenza di notifica per utente**. Questo è intenzionale: i superamenti del budget influenzano i costi, e il responsabile della fatturazione deve esserne informato.

Le notifiche di approvazione rispettano solo l'impostazione di frequenza per-user relativa alle agent-approval. Non tengono conto dell'opt-out più ampio dalle admin-notifications - un utente che ha disattivato le notifiche amministrative riceverà comunque le email di approvazione se è nella lista dei revisori, a meno che la sua frequenza agent-approval non sia impostata su **Disabilitato**.

### Vedi anche

- [Flusso di approvazione](#approval-workflow) per il ciclo di vita completo di un'approvazione.
- [Affinamento dei prompt](#refining-prompts) per il flusso "Continuo ad approvare lo stesso tipo di errore".