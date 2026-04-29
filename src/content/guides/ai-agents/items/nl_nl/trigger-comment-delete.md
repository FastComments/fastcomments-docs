Wordt geactiveerd wanneer een opmerking is verwijderd.

### Context the agent receives

- De opmerking die zojuist is verwijderd - tekst, auteur, pagina.
- Optionele thread / gebruikersgeschiedenis / pagina-context zoals geconfigureerd.

### Belangrijk

- Vindt plaats bij zowel **soft deletes** (waarbij de opmerking verborgen is maar bewaard voor audit) als **hard deletes** (waarbij de opmerking volledig wordt verwijderd). De trigger handler haalt de opmerking op uit de cascade delete pipeline; wat de agent ziet is de laatst bekende staat.
- Zodra een opmerking volledig is verwijderd, zullen tools die daarop gericht zijn (`pin_comment`, `mark_comment_spam`, etc.) op dat comment ID falen.

### Veelvoorkomende toepassingen

- **Audit doorsturen via [Webhooks](#webhooks-overview)** - zend een `trigger.succeeded` event zodat een extern systeem registreert wat er is verwijderd.
- **Memory writes** - laat de agent een [memory note](#tools-overview) vastleggen over een verwijderingspatroon (de verwijderde opmerking was de derde van de gebruiker in 24 uur, enz.).
- **Cross-thread effects** - merk op wanneer een verwijdering de structuur van een thread verandert die de agent eerder heeft samengevat, en overweeg of opnieuw samenvatten nodig is.

### Opmerking over operationele kosten

Als je een site hebt met een hoog aantal verwijderingen (intensieve menselijke moderatie), kan deze trigger vaak afgaan.