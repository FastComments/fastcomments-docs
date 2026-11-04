### Errori 401 Non Autorizzato

Se ricevi errori 401 quando usi l'API autenticata:

1. **Controlla la tua API key**: Assicurati di usare la corretta API key dalla tua dashboard FastComments
2. **Verifica il tenant ID**: Assicurati che il tenant ID corrisponda al tuo account
3. **Formato della API key**: La API key deve essere impostata come header `x-api-key` nella configurazione condivisa:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Uso dell'API sbagliata**: Assicurati di usare `DefaultAPI` (non `PublicAPI`) per le chiamate autenticate

### Problemi con i Token SSO

Se i token SSO non funzionano:

1. **Usa la modalità sicura in produzione**: Usa sempre `FastCommentsSSO.createSecure()` con la tua API key per la produzione
2. **Solo lato server**: Genera i token SSO sicuri sul tuo server, non esporre mai la tua API key ai client
3. **Controlla i dati utente**: Assicurati che tutti i campi richiesti (id, email, username) siano forniti
4. **Scadenza del token**: I token SSO sicuri includono un timestamp e possono scadere. Genera nuovi token quando necessario.

### Errori SSL/TLS

Se riscontri errori SSL/TLS:

1. Assicurati che l'Info.plist della tua app consenta connessioni HTTPS a fastcomments.com
2. Verifica di non usare eccezioni di App Transport Security che potrebbero bloccare la connessione