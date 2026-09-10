---
Un agente di programmazione come Claude Code, Cursor o un assistente basato su MCP può configurare FastComments per te senza che tu debba compilare il modulo di registrazione. Questo è utile quando chiedi a un agente di "aggiungere commenti al mio sito" e non hai ancora un account.

### Come funziona

1. L'agente crea un nuovo account e riceve una chiave API e un link di rivendicazione. La chiave API funziona immediatamente, così l'agente può configurare l'account e installare il widget sul tuo sito.
2. L'agente ti fornisce il link di rivendicazione. Aprilo nel tuo browser, accedi o crea un accesso, e conferma la rivendicazione. L'account è quindi tuo: lo gestisci, la sua fatturazione e le sue chiavi API dal cruscotto. La pagina elenca la chiave API che l'agente possiede così puoi revocarla se non desideri più che l'agente, o chiunque lo utilizzi, abbia accesso.
3. Se nessuno apre il link di rivendicazione entro 72 ore, l'account e la sua chiave vengono eliminati. Chiedi all'agente di crearne uno nuovo.

Fino a quando non viene rivendicato, l'account ha gli stessi limiti di una normale prova gratuita.

### Se hai già un account

Ogni accesso possiede un account. Se apri un link di rivendicazione mentre sei connesso a un account esistente, la pagina ti permette di scegliere:

- **Allega al mio account** rende il nuovo account un tenant gestito di quello a cui sei connesso. Questo richiede un piano a pagamento con white labeling, e l'utilizzo del nuovo tenant viene fatturato al tuo account.
- **Disconnetti e rivendica con un altro accesso** ti disconnette e ti riporta alla pagina di rivendicazione così puoi rivendicarlo con un accesso diverso.

### Per gli autori di agenti

Le istruzioni per gli agenti su [fastcomments.com/agents.md](https://fastcomments.com/agents.md) descrivono la chiamata di creazione dell'account, i campi nella risposta e come consegnare il link di rivendicazione alla persona per cui stai lavorando. La chiamata non richiede una chiave API ed è limitata per indirizzo IP.

---