### 401 Non autorizzato

Se ricevi errori 401 quando usi l'API autenticata:

1. **Controlla la tua chiave API**: Assicurati di usare la chiave API corretta dalla dashboard di FastComments
2. **Verifica il tenant ID**: Assicurati che il tenant ID corrisponda al tuo account
3. **Formato della chiave API**: La chiave API deve essere impostata sul client API:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Uso dell'API sbagliata**: Assicurati di usare `DefaultAPI` (non `PublicAPI`) per chiamate autenticate

### Problemi con i token SSO

Se i token SSO non funzionano:

1. **Usa la modalità sicura in produzione**: Usa sempre `FastCommentsSSO.createSecure()` con la tua chiave API per la produzione
2. **Solo lato server**: Genera i token SSO sicuri sul tuo server, non esporre mai la tua chiave API ai client
3. **Controlla i dati utente**: Assicurati che siano forniti tutti i campi obbligatori (id, email, username)
4. **Scadenza del token**: I token SSO sicuri includono un timestamp e possono scadere. Genera token nuovi quando necessario.

### Errori SSL/TLS

Se incontri errori SSL/TLS:

1. Assicurati che l'Info.plist della tua app consenta connessioni HTTPS a fastcomments.com
2. Verifica di non usare eccezioni di App Transport Security che potrebbero bloccare la connessione