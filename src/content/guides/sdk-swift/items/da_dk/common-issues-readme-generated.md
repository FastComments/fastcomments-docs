### 401 Uautoriseret fejl

Hvis du får 401-fejl, når du bruger den autentificerede API:

1. **Kontroller din API-nøgle**: Sørg for, at du bruger den korrekte API-nøgle fra dit FastComments-dashboard
2. **Bekræft tenant-id'et**: Sørg for, at tenant-id'et matcher din konto
3. **API-nøglens format**: API-nøglen skal sættes på API-klienten:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Brug af den forkerte API**: Sørg for, at du bruger `DefaultAPI` (ikke `PublicAPI`) til autentificerede kald

### Problemer med SSO-tokens

Hvis SSO-tokens ikke virker:

1. **Brug sikker tilstand i produktion**: Brug altid `FastCommentsSSO.createSecure()` med din API-nøgle i produktion
2. **Kun server-side**: Generer sikre SSO-tokens på din server, eksponer aldrig din API-nøgle for klienter
3. **Kontroller brugerdata**: Sørg for, at alle krævede felter (id, email, username) er angivet
4. **Token-udløb**: Sikre SSO-tokens inkluderer et tidsstempel og kan udløbe. Generer nye tokens efter behov.

### SSL/TLS-fejl

Hvis du støder på SSL/TLS-fejl:

1. Sørg for, at din apps Info.plist tillader HTTPS-forbindelser til fastcomments.com
2. Kontroller, at du ikke bruger App Transport Security-undtagelser, som kan blokere forbindelsen