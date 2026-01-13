### 401 – Nicht autorisiert

Wenn Sie 401-Fehler erhalten, wenn Sie die authentifizierte API verwenden:

1. **Überprüfen Sie Ihren API-Schlüssel**: Stellen Sie sicher, dass Sie den korrekten API-Schlüssel aus Ihrem FastComments-Dashboard verwenden
2. **Überprüfen Sie die tenant ID**: Stellen Sie sicher, dass die tenant ID mit Ihrem Konto übereinstimmt
3. **API key format**: Der API-Schlüssel sollte beim API-Client gesetzt werden:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Falsche API verwenden**: Stellen Sie sicher, dass Sie `DefaultAPI` (nicht `PublicAPI`) für authentifizierte Aufrufe verwenden

### SSO-Token-Probleme

Wenn SSO-Token nicht funktionieren:

1. **Sicheren Modus für Produktion verwenden**: Verwenden Sie in der Produktion immer `FastCommentsSSO.createSecure()` mit Ihrem API-Schlüssel
2. **Nur serverseitig**: Generieren Sie sichere SSO-Token auf Ihrem Server, geben Sie Ihren API-Schlüssel niemals an Clients weiter
3. **Benutzerdaten prüfen**: Stellen Sie sicher, dass alle erforderlichen Felder (id, email, username) angegeben sind
4. **Token-Ablauf**: Sichere SSO-Token enthalten einen Zeitstempel und können ablaufen. Generieren Sie bei Bedarf neue Token.

### SSL/TLS-Fehler

Wenn Sie auf SSL/TLS-Fehler stoßen:

1. Stellen Sie sicher, dass die Info.plist Ihrer App HTTPS-Verbindungen zu fastcomments.com erlaubt
2. Prüfen Sie, dass Sie keine App Transport Security-Ausnahmen verwenden, die die Verbindung blockieren könnten