---
De FastComments Rust SDK bestaat uit verschillende modules:

- **Clientmodule** - API-client voor FastComments REST-API's
  - Volledige type-definities voor alle API-modellen
  - Drie API-clients die alle FastComments-methoden omvatten:
    - `default_api` (**DefaultApi**) - API-sleutel-geauthenticeerde methoden voor gebruik aan de serverzijde
    - `public_api` (**PublicApi**) - publieke methoden zonder API-sleutel die veilig vanuit browsers en mobiele apps aangeroepen kunnen worden
    - `moderation_api` (**ModerationApi**) - methoden die het moderatordashboard ondersteunen, inclusief commentaarmoderatie (lijst, telling, zoeken, logs, export), moderatieacties (verwijderen/terugzetten, vlaggen, review/spam/goedkeuringsstatus instellen, stemmen, draad heropenen/sluiten), bans (verbanning op basis van een opmerking, ongedaan maken, pre-ban samenvattingen, ban status/voorkeuren, aantallen geblokkeerde gebruikers), en badges & vertrouwen (toekennen/verwijderen van badges, handmatige badges, get/set trust factor, intern gebruikersprofiel). Elke Moderation-methode accepteert een `sso` parameter zodat de oproep namens een SSO-geauthenticeerde moderator gedaan kan worden.
  - Volledige async/await-ondersteuning met tokio
  - Zie [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) voor gedetailleerde API-documentatie

- **SSO-module** - Server-side Single Sign-On-hulpprogramma's
  - Veilige token-generatie voor gebruikersauthenticatie
  - Ondersteuning voor zowel eenvoudige als veilige SSO-modi
  - HMAC-SHA256-gebaseerde tokenondertekening

- **Kerntypes** - Gedeelde typedefinities en hulpprogramma's
  - Commentaarmodellen en metadata-structuren
  - Gebruikers- en tenantconfiguraties
  - Hulpfuncties voor veelvoorkomende bewerkingen
---