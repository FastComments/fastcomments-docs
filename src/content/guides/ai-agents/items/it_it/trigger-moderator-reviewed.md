---
Si attiva quando un moderatore segna un commento come revisionato.

### Contesto che l'agente riceve

- Il commento.
- L'ID utente **che ha attivato il trigger** - il moderatore che ha revisionato.
- Thread opzionale / cronologia utente / contesto della pagina come configurato.

### Chi attiva questo

Un'azione di un moderatore umano nella pagina di moderazione, nel widget dei commenti o tramite API.

### Usi comuni

- **Inoltro di audit** via [Webhooks](#webhooks-overview).
- **Scritture nella memoria** - registrare una nota di memoria che questo commento è stato revisionato da un umano in modo che altri agenti non lo elaborino due volte.

### Da notare

- "Reviewed" è uno degli stati della coda di moderazione tracciati separatamente da "approved" e "spam". Un commento può essere approved-and-reviewed, approved-but-not-reviewed, ecc. Vedi [Come funzionano le approvazioni](/guide-moderation.html#moderation-approvals) nella guida alla moderazione.
- Questo trigger è ad alta frequenza per tenant con molti moderatori. Iscriviti selettivamente e pianifica il budget di conseguenza.

---