Wordt geactiveerd wanneer het netto aantal stemmen van een opmerking de geconfigureerde drempel bereikt. Netto stemmen is `votesUp - votesDown`.

### Vereiste configuratie

- **Stemdrempel** - geheel getal >= 1. De trigger wordt geactiveerd bij de stem die het netto aantal stemmen precies op dit getal brengt.

Als de drempel 10 is en een opmerking gaat van 9 naar 10 netto stemmen, wordt de trigger één keer geactiveerd. Als een stem het daarna van 10 naar 11 brengt, wordt de trigger **niet** opnieuw geactiveerd — hij vuurt niet bij elke extra stem boven de drempel.

### Context die de agent ontvangt

- De opmerking, met de huidige stemtellingen.
- De **stemrichting** (`up` of `down`) van de stem die de drempeloverschrijding veroorzaakte.
- Optionele thread-/gebruikersgeschiedenis/paginacontext zoals geconfigureerd.

### Opmerkelijk

- Een opmerking die naar 10 gaat, terugvalt naar 9 en opnieuw naar 10 stijgt, zal de trigger twee keer activeren. Er is geen per-opmerking 'eenmaal geactiveerd' status — als je die semantiek nodig hebt, laat de agent dan bij de eerste keer een [geheugennotitie](#tools-overview) aanmaken en controleer daarop bij volgende runs.
- De drempel is altijd een **netto** stemtelling, niet het ruwe aantal upvotes. Een opmerking met 12 up en 2 down heeft netto 10 en activeert de trigger; een opmerking met 10 up en 0 down activeert deze ook.
- Overgangen veroorzaakt door alleen een down-vote zijn mogelijk - een opmerking die van 11 naar 10 gaat door een down-vote activeert ook. De `voteDirection` parameter in de context vertelt de agent vanuit welke richting de drempeloverschrijding kwam.

### Veelvoorkomende toepassingen

- **Pinnen** - de [Top Comment Pinner-sjabloon](#template-top-comment-pinner) is gebouwd rond deze trigger.
- **Promotie / workflows voor uitgelichte opmerkingen** - zend een gebeurtenis via [Webhooks](#webhooks-overview) zodat een extern systeem de opmerking elders op je site kan promoten.
- **Engagement-tracking** - neem een geheugennotitie op over de gebruiker die de opmerking schreef zodat andere agenten weten dat zij populaire inhoud hebben geproduceerd.

### Afstemming

De juiste drempel is specifiek voor je community. Houd [Uitvoeringsgeschiedenis](#run-history) een paar dagen in de gaten met een lage drempel (5) om te zien hoe vaak het afgaat. Verhoog de drempel totdat de activeringsfrequentie overeenkomt met het tempo dat je werkelijk wilt.