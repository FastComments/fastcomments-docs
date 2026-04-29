Si attiva quando un moderatore assegna un badge a un utente.

### Contesto che l'agente riceve

- L'**ID del badge** assegnato.
- L'**ID dell'utente che ha attivato** - il moderatore che ha assegnato il badge.
- Contesto opzionale di thread / cronologia utente / pagina come configurato.

Il fire-site non include **non** un `commentId` nel payload del trigger, anche se il badge è stato assegnato rispetto a un commento specifico.

### Chi lo attiva

Un'azione di un moderatore umano.

### Da notare

- È incluso solo l'ID del badge; l'agente non riceve i metadati del badge (nome, immagine). Se l'agente ha bisogno di ragionare su *quale* badge è stato assegnato, incorpora quel contesto nel [prompt iniziale](#personality-prompt) o nelle [linee guida della community](#community-guidelines).
- Il trigger si attiva una volta per ogni assegnazione del badge, non per utente. Assegnare lo stesso badge a un utente due volte lo attiva due volte (ogni assegnazione è un evento distinto).

### Usi comuni

- **Riconoscimento reciproco** - un agente può pubblicare una risposta "grazie per il grande contributo" quando viene assegnato un badge specifico.
- **Flusso di riconoscimento esterno** tramite [Webhooks](#webhooks-overview) - rispecchiare le assegnazioni di badge nel proprio sistema di coinvolgimento degli utenti.
- **Registrazione nella memoria** - note del tipo "questo utente è un collaboratore riconosciuto" che indicano che i futuri agenti di moderazione dovrebbero tenerne conto nelle loro decisioni.

---