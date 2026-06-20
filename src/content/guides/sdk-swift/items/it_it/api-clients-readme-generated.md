Lo SDK di FastComments fornisce tre client API:

### PublicAPI - Metodi sicuri per il client

Il `PublicAPI` contiene metodi che è sicuro chiamare dal codice lato client (app iOS/macOS). Questi metodi:
- Non richiedono una chiave API
- Possono utilizzare token SSO per l'autenticazione
- Sono soggetti a limiti di richiesta per utente/dispositivo
- Sono adatti per applicazioni rivolte all'utente finale

**Esempio d'utilizzo**: Recuperare e creare commenti nella tua app iOS

### DefaultAPI - Metodi lato server

Il `DefaultAPI` contiene metodi autenticati che richiedono una chiave API. Questi metodi:
- Richiedono la tua chiave API di FastComments
- Devono essere chiamati SOLO dal codice lato server
- Forniscono accesso completo ai dati di FastComments
- Sono soggetti a limiti di richiesta per tenant

**Esempio d'utilizzo**: Operazioni amministrative, esportazione dati in blocco, gestione utenti

### ModerationAPI - Metodi per la dashboard dei moderatori

Il `ModerationAPI` contiene metodi che alimentano la dashboard dei moderatori. Questi metodi coprono:
- **Moderazione dei commenti** - elencare, contare, cercare, recuperare i log ed esportare i commenti
- **Azioni di moderazione** - rimuovere/ripristinare commenti, segnalare, impostare stato di revisione/spam/approvazione, gestire i voti e riaprire/chiudere discussioni
- **Bannamenti** - bannare un utente da un commento, annullare i ban, recuperare riepiloghi pre-ban, controllare lo stato e le preferenze del ban, e leggere il conteggio degli utenti bannati
- **Badge e fiducia** - assegnare/rimuovere badge, elencare i badge manuali, ottenere/impostare il fattore di fiducia di un utente, e leggere il profilo interno di un utente

Ogni metodo di `ModerationAPI` accetta un parametro `sso` in modo che i moderatori possano essere autenticati tramite SSO.

**Esempio d'utilizzo**: Costruire un'esperienza di moderazione per i moderatori della tua comunità

**IMPORTANTE**: Non esporre mai la tua chiave API nel codice lato client. Le chiavi API dovrebbero essere utilizzate solo lato server.