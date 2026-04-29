---
Si attiva quando un moderatore approva un commento.

### Contesto che riceve l'agente

- Il commento appena approvato.
- L'**ID utente che ha attivato** - il moderatore che ha approvato.
- Contesto opzionale della discussione / cronologia utente / pagina come configurato.

### Chi lo attiva

Un'azione di un moderatore umano.

### Note

- Un commento "approvato" è un commento **visibile** nella terminologia FastComments. Vedi [Come funzionano le approvazioni](/guide-moderation.html#moderation-approvals) nella guida alla moderazione per la distinzione tra approvato/non approvato e revisionato/non revisionato.
- Il trigger si attiva sulle **transizioni** di approvazione: un commento che passa da non approvato ad approvato lo attiva; un commento già approvato che viene salvato nuovamente non lo fa.
- Per i tenant in cui i commenti sono approvati automaticamente per impostazione predefinita, questo trigger si attiva solo quando un moderatore ri-approva esplicitamente un commento precedentemente nascosto.

### Usi comuni

- **Benvenuto / coinvolgimento** - un agente può rispondere a chi commenta per la prima volta nel momento in cui un moderatore li approva, anziché al momento della pubblicazione.
- **Coordinamento tra agenti** - se un agente separato aveva contrassegnato il commento per la revisione, l'approvazione è il segnale che la revisione umana è completata.
- **Registro di controllo** tramite [Webhooks](#webhooks-overview).

---