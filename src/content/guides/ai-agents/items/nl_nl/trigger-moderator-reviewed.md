Wordt geactiveerd wanneer een moderator een commentaar als beoordeeld markeert.

### Context die de agent ontvangt

- De opmerking.
- De **triggerende gebruiker-ID** - de moderator die heeft beoordeeld.
- Optionele thread- / gebruikersgeschiedenis- / pagina-context zoals geconfigureerd.

### Wie activeert dit

Een handeling door een menselijke moderator op de moderatiepagina, de commentaar-widget, of via de API.

### Veelvoorkomende toepassingen

- **Audit doorsturen** via [Webhooks](#webhooks-overview).
- **Geheugenopslagen** - leg een geheugenopmerking vast dat deze opmerking door een mens is beoordeeld zodat andere agenten hem niet dubbel verwerken.

### Opmerkingen

- 'Beoordeeld' is een van de moderatiewachtrijstatussen die apart wordt bijgehouden van 'goedgekeurd' en 'spam'. Een opmerking kan zowel goedgekeurd-en-beoordeeld zijn, goedgekeurd-maar-niet-beoordeeld, enz. Zie [Hoe goedkeuringen werken](/guide-moderation.html#moderation-approvals) in de moderatiehandleiding.
- Deze trigger komt vaak voor bij tenants met veel moderators. Schrijf je selectief in en houd hier rekening mee in je budgettering.

---