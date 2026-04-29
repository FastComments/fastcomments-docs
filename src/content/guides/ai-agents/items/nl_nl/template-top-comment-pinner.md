**Template-ID:** `top_comment_pinner`

De Top Comment Pinner houdt top-level reacties in de gaten die een stemdrempel overschrijden en zet ze vast, waarbij iets dat eerder in dezelfde discussiedraad was gepind wordt vervangen.

De ingebouwde prompt instrueert de agent om replies over te slaan (vastzetten werkt op discussiedraden, dus het vastzetten van een reply is zelden nuttig) en om promotionele content te filteren (zodat de agent geen populaire link-spam groter maakt).

### Triggers

- **Een reactie overschrijdt een stemdrempel** (`COMMENT_VOTE_THRESHOLD`, standaard stemdrempel: 10).

De trigger gaat af wanneer de netto stemmen van de reactie (`up - down`) de geconfigureerde drempel bereiken. Pas het getal aan op het bewerkformulier op basis van hoe actief je discussiedraden zijn — 10 is een verstandige standaard voor matig actieve sites.

### Toegestane tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Vastzetten is niet-destructief — het kan onmiddellijk ongedaan worden gemaakt — dus deze template draait meestal zonder goedkeuringen.

### Aanbevolen aanvullingen voordat u live gaat

- **Vink "Inclusief ouderreactie en eerdere antwoorden in dezelfde thread" aan** in [Contextopties](#context-options). Zonder context van de discussiedraad kan de agent niet betrouwbaar bepalen of er al een gepinde reactie is om te ontpinnen.
- **Pas de stemdrempel aan** op jouw site. In drukke discussiedraden gebeurt 10 te vaak; in rustige discussiedraden gebeurt 10 misschien nooit.
- **Overweeg te beperken op basis van URL** als je alleen gepinde reacties wilt in bepaalde secties van je site — bijvoorbeeld nieuwsdiscussies, maar niet aankondigingsdiscussies.

### Opmerking over dubbele pinningen

De prompt van de agent instrueert om eerst te ontpinnen voordat er gepind wordt, maar als het model die stap mist handhaaft het platform zelf geen regel van één gepinde reactie per discussiedraad (je kunt er meerdere hebben). Als dubbele pinningen een probleem zijn op je site, zet `pin_comment` achter goedkeuring en beoordeel elke pin — of schrijf een striktere prompt.