De FastComments SDK biedt drie API-clients:

### PublicAPI - Client-Safe Methoden

De `PublicAPI` bevat methoden die veilig kunnen worden aangeroepen vanuit client-side code (iOS/macOS-apps). Deze methoden:
- Vereisen geen API-sleutel
- Kunnen SSO-tokens gebruiken voor authenticatie
- Zijn rate-limited per gebruiker/apparaat
- Zijn geschikt voor applicaties gericht op eindgebruikers

**Voorbeeld use case**: Het ophalen en maken van opmerkingen in je iOS-app

### DefaultAPI - Server-Side Methoden

De `DefaultAPI` bevat geauthenticeerde methoden die een API-sleutel vereisen. Deze methoden:
- Vereisen je FastComments API-sleutel
- Mogen ALLEEN worden aangeroepen vanuit server-side code
- Bieden volledige toegang tot je FastComments-gegevens
- Zijn rate-limited per tenant

**Voorbeeld use case**: Administratieve bewerkingen, bulkdata-export, gebruikersbeheer

### ModerationAPI - Moderator Dashboard Methoden

De `ModerationAPI` biedt een uitgebreide suite van live en snelle moderatie-API's. Elke `ModerationAPI`-methode accepteert een `sso`-parameter en kan authenticeren via SSO of een FastComments.com-sessiecookie.

**Voorbeeld use case**: Het bouwen van een moderatie-ervaring voor moderators van jouw community

**BELANGRIJK**: Stel je API-sleutel nooit bloot in client-side code. API-sleutels mogen alleen server-side worden gebruikt.