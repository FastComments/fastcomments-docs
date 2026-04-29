Udløses, når en moderator markerer en kommentar som gennemset.

### Kontekst som agenten modtager

- Kommentaren.
- Den **udløsende bruger-id** - moderatoren, som gennemgik.
- Valgfri tråd / brugerhistorik / sidekontekst som konfigureret.

### Hvem udløser dette

En handling foretaget af en menneskelig moderator på moderationssiden, i kommentar-widgeten eller via API.

### Almindelige anvendelser

- **Videresendelse af revisionslog** via [Webhooks](#webhooks-overview).
- **Hukommelsesskrivninger** - registrer en hukommelsesnote om, at denne kommentar blev gennemset af et menneske, så andre agenter ikke behandler den dobbelt.

### Bemærk

- "Gennemset" er en af moderationkøens tilstande, der spores separat fra "godkendt" og "spam". En kommentar kan være godkendt-og-gennemset, godkendt-men-ikke-gennemset osv. Se [Hvordan godkendelser fungerer](/guide-moderation.html#moderation-approvals) i moderationsguiden.
- Denne trigger er højfrekvent hos lejere med mange moderatorer. Abonner selektivt og tilpas budgettet derefter.