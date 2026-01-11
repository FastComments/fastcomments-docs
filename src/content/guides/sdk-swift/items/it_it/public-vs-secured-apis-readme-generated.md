Lo SDK di FastComments fornisce due tipi di endpoint API:

### PublicAPI - Endpoint sicuri lato client

The `PublicAPI` contains endpoints that are safe to call from client-side code (app iOS/macOS). These endpoints:
- Do not require an API key
- Can use SSO tokens for authentication
- Are rate-limited per user/device
- Are suitable for end-user facing applications

**Esempio d'uso**: Recupero e creazione di commenti nella tua app iOS

### DefaultAPI - Endpoint lato server

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- Require your FastComments API key
- Should ONLY be called from server-side code
- Provide full access to your FastComments data
- Are rate-limited per tenant

**Esempio d'uso**: Operazioni amministrative, esportazione massiva di dati, strumenti di moderazione

**IMPORTANTE**: Non esporre mai la tua chiave API nel codice lato client. Le chiavi API devono essere utilizzate solo lato server.