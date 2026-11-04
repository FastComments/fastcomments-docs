### 401 Unauthorized-Fehler

Wenn Sie 401-Fehler bei der Verwendung der authentifizierten API erhalten:

1. **Überprüfen Sie Ihren API-Schlüssel**: Stellen Sie sicher, dass Sie den korrekten API-Schlüssel aus Ihrem FastComments-Dashboard verwenden
2. **Überprüfen Sie die Tenant-ID**: Stellen Sie sicher, dass die Tenant-ID mit Ihrem Konto übereinstimmt
3. **Format des API-Schlüssels**: Der API-Schlüssel sollte als `x-api-key`-Header in der gemeinsamen Konfiguration gesetzt sein:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Verwendung der falschen API**: Stellen Sie sicher, dass Sie `DefaultAPI` (nicht `PublicAPI`) für authentifizierte Aufrufe verwenden

### Probleme mit SSO-Tokens

Wenn SSO-Tokens nicht funktionieren:

1. **Verwenden Sie den sicheren Modus für die Produktion**: Verwenden Sie für die Produktion immer `FastCommentsSSO.createSecure()` mit Ihrem API-Schlüssel
2. **Nur serverseitig**: Generieren Sie sichere SSO-Tokens auf Ihrem Server und geben Sie Ihren API-Schlüssel niemals an Clients weiter
3. **Benutzerdaten prüfen**: Stellen Sie sicher, dass alle erforderlichen Felder (id, email, username) angegeben sind
4. **Token-Ablauf**: Sichere SSO-Tokens enthalten einen Zeitstempel und können ablaufen. Generieren Sie bei Bedarf neue Tokens.

### SSL/TLS-Fehler

Wenn Sie auf SSL/TLS-Fehler stoßen:

1. Stellen Sie sicher, dass die Info.plist Ihrer App HTTPS-Verbindungen zu fastcomments.com erlaubt
2. Prüfen Sie, ob Sie keine App Transport Security-Ausnahmen verwenden, die die Verbindung blockieren könnten