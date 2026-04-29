---
Si attiva quando un moderatore contrassegna un commento come spam.

### Contesto che l'agente riceve

- Il commento, con il flag post-azione `Is Spam`.
- L'**ID utente che ha innescato** - il moderatore che ha agito.
- Cronologia opzionale della discussione / dell'utente / del contesto della pagina come configurato.

### Chi lo attiva

Un'azione di un moderatore umano. I contrassegni di spam generati dall'agente (via [`mark_comment_spam`](#tools-overview)) non attivano questo trigger.

### Usi comuni

- **Registrazione della memoria** - far sì che un agente salvi una nota di memoria sull'utente segnalato come spam (ad es., "in precedenza segnalato come spam per X dal moderatore") in modo che i futuri agenti di moderazione abbiano il contesto.
- **Applicazione a livello utente** - il contrassegno di un commento come spam da parte di un moderatore potrebbe essere il segnale per un agente di emettere anche un avviso o una breve sospensione, subordinato ad approvazione.
- **Replica su sistema esterno** tramite [Webhooks](#webhooks-overview).

---