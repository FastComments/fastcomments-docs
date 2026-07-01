### 401 Uautoriserede fejl

Hvis du får 401-fejl, når du bruger den autentificerede API:

1. **Kontrollér din API-nøgle**: Sørg for, at du bruger den korrekte API-nøgle fra dit FastComments-dashboard
2. **Bekræft lejer-ID'et**: Sørg for, at lejer-ID'et stemmer overens med din konto
3. **API-nøgleformat**: API-nøglen skal sættes som `x-api-key` headeren i den delte konfiguration:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Brug af den forkerte API**: Sørg for, at du bruger `DefaultAPI` (ikke `PublicAPI`) til autentificerede kald

### SSO-tokenproblemer

Hvis SSO-tokens ikke virker:

1. **Brug sikker tilstand til produktion**: Brug altid `FastCommentsSSO.createSecure()` med din API-nøgle til produktion
2. **Kun på serversiden**: Generér sikre SSO-tokens på din server, og udsæt aldrig din API-nøgle for klienter
3. **Kontrollér brugerdata**: Sørg for, at alle krævede felter (id, e‑mail, brugernavn) er angivet
4. **Tokenudløb**: Sikker SSO-tokens indeholder et tidsstempel og kan udløbe. Generér friske tokens efter behov.

### SSL/TLS-fejl

Hvis du støder på SSL/TLS-fejl:

1. Sørg for, at din apps Info.plist tillader HTTPS-forbindelser til fastcomments.com
2. Kontrollér, at du ikke bruger App Transport Security-undtagelser, som kan blokere forbindelsen.