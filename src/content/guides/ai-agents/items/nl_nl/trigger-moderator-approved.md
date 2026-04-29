Wordt geactiveerd wanneer een moderator een reactie goedkeurt.

### Context die de agent ontvangt

- De zojuist goedgekeurde reactie.
- De **triggering user ID** - de moderator die goedkeurde.
- Optionele thread / gebruikersgeschiedenis / pagina-context zoals geconfigureerd.

### Wie dit activeert

Een handeling van een menselijke moderator.

### Opmerkingen

- Een "approved" reactie is een **zichtbare** reactie in FastComments-terminologie. Zie [Hoe goedkeuringen werken](/guide-moderation.html#moderation-approvals) in de moderatiehandleiding voor het onderscheid tussen approved/unapproved en reviewed/unreviewed.
- De trigger wordt geactiveerd bij goedkeurings **transities**: een reactie die van unapproved naar approved gaat activeert hem; een reactie die al approved was en opnieuw wordt opgeslagen niet.
- Voor tenants waar reacties standaard auto-approved zijn, wordt deze trigger alleen geactiveerd wanneer een moderator expliciet een eerder verborgen reactie opnieuw goedkeurt.

### Veelvoorkomende toepassingen

- **Welcome / engagement** - een agent kan reageren op reageerders die voor het eerst posten op het moment dat een moderator hen goedkeurt, in plaats van bij het plaatsen.
- **Cross-agent coordination** - als een aparte agent de reactie had gemarkeerd voor review, is de goedkeuring het teken dat menselijke beoordeling is afgerond.
- **Auditspoor** via [Webhooks](#webhooks-overview).

---