L'SDK FastComments fornisce due tipi di endpoint API:

### PublicAPI - Endpoint Sicuri per il Client

La `PublicAPI` contiene endpoint che è sicuro chiamare dal codice lato client (app iOS/macOS). Questi endpoint:
- Non richiedono una chiave API
- Possono utilizzare token SSO per l'autenticazione
- Sono soggetti a limitazioni di frequenza per utente/dispositivo
- Sono adatti per applicazioni rivolte agli utenti finali

**Esempio di caso d'uso**: Recupero e creazione di commenti nella tua app iOS

### DefaultAPI - Endpoint Lato Server

La `DefaultAPI` contiene endpoint autenticati che richiedono una chiave API. Questi endpoint:
- Richiedono la tua chiave API FastComments
- Devono essere chiamati SOLO da codice lato server
- Forniscono accesso completo ai tuoi dati FastComments
- Sono soggetti a limitazioni di frequenza per tenant

**Esempio di caso d'uso**: Operazioni amministrative, esportazione di dati in blocco, strumenti di moderazione

**IMPORTANTE**: Non esporre mai la tua chiave API nel codice lato client. Le chiavi API devono essere utilizzate solo lato server.