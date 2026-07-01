### 401 Unauthorized-Fehler

If you're getting 401 errors when using the authenticated API:

1. **Check your API key**: Ensure you're using the correct API key from your FastComments dashboard  
   → **Überprüfen Sie Ihren API‑Schlüssel**: Stellen Sie sicher, dass Sie den richtigen API‑Schlüssel aus Ihrem FastComments‑Dashboard verwenden
2. **Verify the tenant ID**: Make sure the tenant ID matches your account  
   → **Überprüfen Sie die Mandanten‑ID**: Stellen Sie sicher, dass die Mandanten‑ID mit Ihrem Konto übereinstimmt
3. **API key format**: The API key should be set as the `x-api-key` header on the shared configuration:  

   ```swift
   FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
   ```
4. **Using the wrong API**: Make sure you're using `DefaultAPI` (not `PublicAPI`) for authenticated calls  
   → **Verwendung der falschen API**: Stellen Sie sicher, dass Sie `DefaultAPI` (nicht `PublicAPI`) für authentifizierte Aufrufe verwenden

### SSO‑Token‑Probleme

If SSO tokens aren't working:

1. **Use secure mode for production**: Always use `FastCommentsSSO.createSecure()` with your API key for production  
   → **Verwenden Sie den sicheren Modus für die Produktion**: Verwenden Sie immer `FastCommentsSSO.createSecure()` mit Ihrem API‑Schlüssel für die Produktion
2. **Server-side only**: Generate secure SSO tokens on your server, never expose your API key to clients  
   → **Nur serverseitig**: Generieren Sie sichere SSO‑Token auf Ihrem Server und geben Sie Ihren API‑Schlüssel niemals an Clients weiter
3. **Check user data**: Ensure all required fields (id, email, username) are provided  
   → **Benutzerdaten prüfen**: Stellen Sie sicher, dass alle erforderlichen Felder (ID, E‑Mail, Benutzername) bereitgestellt werden
4. **Token expiration**: Secure SSO tokens include a timestamp and may expire. Generate fresh tokens as needed.  
   → **Token‑Ablauf**: Sichere SSO‑Token enthalten einen Zeitstempel und können ablaufen. Generieren Sie bei Bedarf neue Token.

### SSL/TLS‑Fehler

If you encounter SSL/TLS errors:

1. Ensure your app's Info.plist allows HTTPS connections to fastcomments.com  
   → Stellen Sie sicher, dass die Info.plist Ihrer App HTTPS‑Verbindungen zu fastcomments.com erlaubt
2. Check that you're not using App Transport Security exceptions that might block the connection  
   → Überprüfen Sie, dass Sie keine App Transport Security‑Ausnahmen verwenden, die die Verbindung blockieren könnten