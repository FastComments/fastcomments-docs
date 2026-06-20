De FastComments SDK biedt drie API-clients:

### PublicAPI - Client-Safe Methods

De `PublicAPI` bevat methoden die veilig zijn om vanuit client-side code (iOS/macOS apps) aan te roepen. Deze methoden:
- Vereisen geen API key
- Kunnen SSO tokens gebruiken voor authenticatie
- Hebben verzoekslimieten per gebruiker/apparaat
- Zijn geschikt voor eindgebruikersgerichte applicaties

**Example use case**: Reacties ophalen en aanmaken in je iOS-app

### DefaultAPI - Server-Side Methods

De `DefaultAPI` bevat geauthenticeerde methoden die een API key vereisen. Deze methoden:
- Vereisen je FastComments API key
- Moeten ALLEEN vanuit server-side code worden aangeroepen
- Bieden volledige toegang tot je FastComments-gegevens
- Hebben verzoekslimieten per tenant

**Example use case**: Administratieve bewerkingen, bulkexport van gegevens, gebruikersbeheer

### ModerationAPI - Moderator Dashboard Methods

De `ModerationAPI` bevat methoden die het moderatordashboard aandrijven. Deze methoden omvatten:
- **Comment moderation** - reacties opnoemen, tellen, zoeken, logboeken ophalen en reacties exporteren
- **Moderation actions** - reacties verwijderen/herstellen, markeren, review-/spam-/goedkeuringsstatus instellen, stemmen beheren en discussiedraden heropenen/sluiten
- **Bans** - een gebruiker van een reactie verbannen, verbanningen ongedaan maken, pre-ban-samenvattingen ophalen, verbanningsstatus en voorkeuren controleren en aantallen verbannen gebruikers lezen
- **Badges & trust** - badges toekennen/verwijderen, handmatige badges opnoemen, de vertrouwensfactor van een gebruiker ophalen/instellen, en het interne profiel van een gebruiker lezen

Every `ModerationAPI` method accepts an `sso` parameter so moderators can be authenticated via SSO.

**Example use case**: Een moderatie-ervaring bouwen voor de moderators van je community

**BELANGRIJK**: Blootstel je API key nooit in client-side code. API keys mogen alleen server-side worden gebruikt.