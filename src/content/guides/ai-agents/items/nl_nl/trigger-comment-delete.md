Wordt geactiveerd wanneer een reactie wordt verwijderd.

### Context die de agent ontvangt

- De reactie die zojuist is verwijderd - tekst, auteur, pagina.
- Optionele thread-/gebruikersgeschiedenis-/paginacontext zoals geconfigureerd.

### Opmerkelijk

- Wordt geactiveerd voor zowel **soft deletes** (waarbij de reactie verborgen blijft maar bewaard wordt voor audit) als **hard deletes** (waarbij de reactie volledig wordt verwijderd). De trigger-handler haalt de reactie op uit de cascade delete-pipeline; wat de agent ziet is de laatst bekende status.
- Zodra een reactie volledig is verwijderd, zullen tools die erop gericht zijn (`pin_comment`, `mark_comment_spam`, enz.) op die reactie-ID mislukken.

### Veelvoorkomende toepassingen

- **Audit forwarding via [Webhooks](#webhooks-overview)** - zend een `trigger.succeeded`-event uit zodat een extern systeem registreert wat er is verwijderd.
- **Geheugenwrites** - laat de agent een [geheugennotitie](#tools-overview) vastleggen over een verwijderpatroon (de verwijderde reactie was de derde van de gebruiker in 24 uur, enz.).
- **Cross-thread effecten** - merk op wanneer een verwijdering de structuur wijzigt van een thread die de agent eerder heeft samengevat, en overweeg of opnieuw moet worden samengevat.

### Kostennotitie

Als je een site hebt met een hoog verwijderingsvolume (veel menselijke moderatie), kan deze trigger vaak worden geactiveerd.