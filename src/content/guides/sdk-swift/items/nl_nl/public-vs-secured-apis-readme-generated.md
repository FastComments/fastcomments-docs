De FastComments SDK biedt twee typen API-eindpunten:

### PublicAPI - Client-veilige eindpunten

De `PublicAPI` bevat eindpunten die veilig zijn om aan te roepen vanuit client-side code (iOS/macOS apps). Deze eindpunten:
- Vereisen geen API key
- Kunnen SSO tokens gebruiken voor authenticatie
- Hebben rate-limieten per gebruiker/apparaat
- Zijn geschikt voor toepassingen gericht op eindgebruikers

**Voorbeeldgebruik**: Het ophalen en aanmaken van reacties in je iOS app

### DefaultAPI - Server-side eindpunten

De `DefaultAPI` bevat geauthenticeerde eindpunten die een API key vereisen. Deze eindpunten:
- Vereisen je FastComments API key
- Moeten ALLEEN worden aangeroepen vanuit server-side code
- Bieden volledige toegang tot je FastComments-gegevens
- Hebben rate-limieten per tenant

**Voorbeeldgebruik**: Administratieve bewerkingen, bulkgegevensexport, moderatietools

**BELANGRIJK**: Stel je API key nooit bloot in client-side code. API keys mogen alleen server-side worden gebruikt.