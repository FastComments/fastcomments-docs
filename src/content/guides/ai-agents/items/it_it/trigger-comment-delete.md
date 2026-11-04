Viene attivato quando un commento viene eliminato.

### Contesto ricevuto dall'agente

- Il commento appena eliminato - testo, autore, pagina.
- Cronologia opzionale del thread / dell'utente / contesto della pagina, come configurato.

### Note importanti

- Viene attivato sia per le **eliminazioni soft** (dove il commento viene nascosto ma conservato per audit) sia per le **eliminazioni hard** (dove il commento viene rimosso completamente). L'handler del trigger recupera il commento dalla pipeline di eliminazione a cascata; ciò che l'agente vede è l'ultimo stato noto.
- Una volta che un commento è stato eliminato completamente, i tool che lo riguardano (`pin_comment`, `mark_comment_spam`, ecc.) su quell'ID di commento falliranno.

### Usi comuni

- **Inoltro per audit tramite [Webhook](#webhooks-overview)** - emette un evento `trigger.succeeded` affinché un sistema esterno registri cosa è stato eliminato.
- **Scritture in memoria** - fai in modo che l'agente registri una [nota di memoria](#tools-overview) su un pattern di eliminazione (il commento eliminato era il terzo dell'utente in 24 ore, ecc.).
- **Effetti cross-thread** - nota quando un'eliminazione modifica la struttura di un thread che l'agente ha precedentemente riassunto, e valuta se riassumerlo nuovamente.

### Nota sui costi operativi

Se hai un sito con un alto volume di eliminazioni (moderazione umana intensa), questo trigger può attivarsi frequentemente.