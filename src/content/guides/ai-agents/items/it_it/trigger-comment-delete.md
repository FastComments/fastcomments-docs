Si attiva quando un commento viene eliminato.

### Context the agent receives

- Il commento appena eliminato - testo, autore, pagina.
- Cronologia del thread / dell'utente / contesto della pagina opzionale come configurato.

### Da notare

- Si attiva sia per le **cancellazioni soft** (dove il commento è nascosto ma conservato per audit) sia per le **cancellazioni hard** (dove il commento è completamente rimosso). Il gestore del trigger risolve il commento dalla pipeline di cancellazione a cascata; ciò che l'agente vede è l'ultimo stato noto.
- Una volta che un commento è completamente eliminato, gli strumenti che lo prendono di mira (`pin_comment`, `mark_comment_spam`, etc.) su quell'ID di commento falliranno.

### Usi comuni

- **Inoltro di audit tramite [Webhooks](#webhooks-overview)** - emettere un evento `trigger.succeeded` in modo che un sistema esterno registri ciò che è stato eliminato.
- **Scritture nella memoria** - far sì che l'agente registri una [nota di memoria](#tools-overview) su un modello di eliminazione (il commento eliminato era il terzo dell'utente nelle 24 ore, ecc.).
- **Effetti tra thread** - notare quando un'eliminazione cambia la struttura di un thread che l'agente ha precedentemente riassunto, e valutare se riassumere di nuovo.

### Nota sul costo operativo

Se hai un sito con un alto volume di eliminazioni (moderazione umana intensa), questo trigger può attivarsi frequentemente.