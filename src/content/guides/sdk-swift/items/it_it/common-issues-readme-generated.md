### Errori 401 Non Autorizzati

If you're getting 401 errors when using the authenticated API:

1. **Controlla la tua chiave API**: Assicurati di utilizzare la chiave API corretta dal tuo cruscotto FastComments
2. **Verifica l'ID tenant**: Assicurati che l'ID tenant corrisponda al tuo account
3. **Formato della chiave API**: La chiave API dovrebbe essere impostata come intestazione `x-api-key` nella configurazione condivisa:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Utilizzo della API sbagliata**: Assicurati di utilizzare `DefaultAPI` (non `PublicAPI`) per le chiamate autenticate

### Problemi con i Token SSO

If SSO tokens aren't working:

1. **Usa la modalità sicura per la produzione**: Usa sempre `FastCommentsSSO.createSecure()` con la tua chiave API per la produzione
2. **Solo lato server**: Genera token SSO sicuri sul tuo server, non esporre mai la tua chiave API ai client
3. **Controlla i dati dell'utente**: Assicurati che tutti i campi richiesti (id, email, username) siano forniti
4. **Scadenza del token**: I token SSO sicuri includono un timestamp e possono scadere. Genera token nuovi quando necessario.

### Errori SSL/TLS

If you encounter SSL/TLS errors:

1. Assicurati che il file Info.plist della tua app consenta connessioni HTTPS a fastcomments.com
2. Verifica di non utilizzare eccezioni di App Transport Security che potrebbero bloccare la connessione