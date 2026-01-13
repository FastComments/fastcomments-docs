### 401 Ongeautoriseerde fouten

Als je 401-fouten krijgt bij het gebruik van de geauthenticeerde API:

1. **Controleer je API-sleutel**: Zorg ervoor dat je de juiste API-sleutel uit je FastComments-dashboard gebruikt  
2. **Controleer de tenant-ID**: Zorg dat de tenant-ID overeenkomt met je account  
3. **API-sleutel formaat**: De API-sleutel moet op de API-client worden ingesteld:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Verkeerde API gebruiken**: Zorg ervoor dat je `DefaultAPI` (niet `PublicAPI`) gebruikt voor geauthenticeerde aanroepen

### SSO-tokenproblemen

Als SSO-tokens niet werken:

1. **Gebruik veilige modus in productie**: Gebruik altijd `FastCommentsSSO.createSecure()` met je API-sleutel voor productie  
2. **Alleen server-side**: Genereer veilige SSO-tokens op je server, maak je API-sleutel nooit openbaar aan clients  
3. **Controleer gebruikersgegevens**: Zorg ervoor dat alle vereiste velden (id, email, username) zijn meegegeven  
4. **Tokenverval**: Veilige SSO-tokens bevatten een tijdstempel en kunnen verlopen. Genereer indien nodig nieuwe tokens.

### SSL/TLS-fouten

Als je SSL/TLS-fouten tegenkomt:

1. Zorg ervoor dat de Info.plist van je app HTTPS-verbindingen naar fastcomments.com toestaat  
2. Controleer of je geen App Transport Security-uitzonderingen gebruikt die de verbinding kunnen blokkeren