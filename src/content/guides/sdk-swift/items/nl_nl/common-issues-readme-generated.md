### 401 Niet-geautoriseerde fouten

Als je 401-fouten krijgt bij het gebruik van de geauthentiseerde API:

1. **Controleer je API‑sleutel**: Zorg ervoor dat je de juiste API‑sleutel van je FastComments-dashboard gebruikt  
2. **Verifieer de tenant‑ID**: Zorg ervoor dat de tenant‑ID overeenkomt met je account  
3. **API‑sleutelformaat**: De API‑sleutel moet ingesteld worden als de `x-api-key` header in de gedeelde configuratie:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Gebruik je de verkeerde API**: Zorg ervoor dat je `DefaultAPI` (niet `PublicAPI`) gebruikt voor geauthentificeerde oproepen  

### SSO‑tokenproblemen

Als SSO‑tokens niet werken:

1. **Gebruik de veilige modus voor productie**: Gebruik altijd `FastCommentsSSO.createSecure()` met je API‑sleutel voor productie  
2. **Alleen server‑side**: Genereer veilige SSO‑tokens op je server, en exposeer je API‑sleutel nooit aan clients  
3. **Controleer gebruikersgegevens**: Zorg ervoor dat alle verplichte velden (id, e‑mail, gebruikersnaam) zijn opgegeven  
4. **Token‑verval**: Veilige SSO‑tokens bevatten een tijdstempel en kunnen verlopen. Genereer indien nodig nieuwe tokens.  

### SSL/TLS‑fouten

Als je SSL/TLS‑fouten tegenkomt:

1. Zorg ervoor dat het Info.plist‑bestand van je app HTTPS‑verbindingen naar fastcomments.com toestaat  
2. Controleer of je geen App Transport Security‑uitzonderingen gebruikt die de verbinding kunnen blokkeren