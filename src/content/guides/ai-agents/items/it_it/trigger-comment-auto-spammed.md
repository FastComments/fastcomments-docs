Si attiva quando un commento viene automaticamente segnalato come spam dall'engine antispam integrato di FastComments - **non** da un moderatore e non da un altro agente.

### Contesto che riceve l'agente

- Il commento auto-segnalato come spam.
- Eventuale thread / cronologia utente / contesto della pagina come configurato.

### Chi attiva questo

La pipeline antispam della piattaforma. Vedi [Rilevamento dello spam](/guide-moderation.html#spam-detection) nella guida alla moderazione per maggiori dettagli.

### Utilizzi comuni

- **Moderazione di secondo livello** - il motore antispam ha un alto richiamo ma precisione imperfetta; un agente addestrato sullo stile specifico della tua community può individuare i falsi positivi. L'agente può richiamare l'azione per rimuovere la segnalazione su un commento classificato erroneamente.
- **Revoca automatica del ban** - se il tuo tenant banna aggressivamente gli account nuovi per spam, un agente su questo trigger può esaminare e rimuovere palesi falsi positivi prima che un umano li veda.

### Note importanti

- Il trigger **non** si attiva per lo spam contrassegnato da un moderatore (usa [Trigger: Spam contrassegnato dal moderatore](#trigger-moderator-spammed)) né per lo spam contrassegnato da un altro agente.
- Un commento che viene auto-segnalato come spam e poi successivamente contrassegnato come Not Spam da un moderatore non riattiva il trigger.
- Sottoscrivere questo trigger è più utile nei tenant in cui la modalità auto-spam del motore è abilitata nelle Impostazioni di Moderazione. Altrimenti il trigger non si attiverà.