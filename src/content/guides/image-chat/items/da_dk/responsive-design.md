### Percentage-Based Positioning

Image Chat bruger procentbaserede koordinater i stedet for pixelkoordinater til at placere chatmarkører på billeder. Når en bruger klikker på et billede, konverterer widgetten pixelkoordinaterne for klikket til procenter af billedets bredde og højde. Denne tilgang sikrer, at markørerne forbliver korrekt placeret uanset hvordan billedet vises.

For eksempel, hvis en bruger klikker 250 pixels fra venstre kant af et 1000px bredt billede, gemmer widgetten dette som 25% fra venstre. Når en anden bruger ser det samme billede ved 500px bredde på en mobil enhed, vises markøren 125 pixels fra venstre, hvilket stadig er 25% af bredden.

### Benefits for Responsive Layouts

Dette procent-system får Image Chat til at fungere problemfrit på tværs af alle enhedsstørrelser og orienteringer. Dine billeder kan blive vist i forskellige størrelser afhængigt af skærmbredden, CSS-regler eller containerbegrænsninger, men markørerne aligner altid korrekt med de tilsigtede placeringer.

Brugere på stationære computere med store skærme ser markørerne i de samme relative positioner som brugere på smartphones med små skærme. Markørerne skalerer proportionalt med selve billedet.

### Image Scaling and Aspect Ratio

Så længe dit billede bevarer sit billedforhold ved skalering (hvilket er standardadfærden i browseren), forbliver de procentbaserede markører perfekt justerede. Widgetten antager, at billeder skalerer proportionalt og beregner positioner baseret på denne antagelse.

Hvis du anvender CSS, der fordrejer billedets billedforhold (som ved brug af `object-fit: cover` med specifikke dimensioner), kan markørpositionerne muligvis ikke alignere korrekt. For bedst resultat, lad billederne skalere naturligt eller brug `object-fit: contain` for at bevare billedforholdet.

### Chat Square Sizing

Den visuelle størrelse af chatmarkørerne er også procentbaseret. The `chatSquarePercentage` configuration option defaults to 5%, meaning each square is 5% of the image width. Dette sikrer en konsekvent visuel vægt på tværs af forskellige billedstørrelser.

På et 1000px bredt billede med standardindstillingen på 5% er markørerne 50px kvadratiske. På et 500px bredt billede er de samme markører 25px kvadratiske. De forbliver proportionale i forhold til billedets størrelse.

### Mobile Behavior

På skærme under 768px bredde skifter Image Chat til et mobiloptimeret layout. Chatvinduer åbnes i fuld skærm i stedet for at flyde ved siden af markøren. Dette giver bedre brugervenlighed på små skærme, hvor flydende vinduer ville dække for meget af billedet.

Selve markørerne forbliver synlige og klikbare på deres procentbaserede positioner. Brugere kan stadig se, hvor alle diskussioner er placeret, og trykke på markører for at åbne chatgrænsefladen i fuld skærm.

### Dynamic Image Loading

Det procentbaserede system fungerer korrekt selv når billeder indlæses asynkront eller ændrer størrelse efter siden er indlæst. Widgetten overvåger billedelementet og genberegner markørpositioner hver gang billedets dimensioner ændrer sig.

Hvis du bruger lazy-loading af billeder eller implementerer responsive billeder med forskellige størrelser ved forskellige brudpunkter, justerer markørerne sig automatisk, når billedets størrelse ændres.

### Cross-Device Consistency

Fordi koordinater gemmes som procenter i databasen, vises en diskussion oprettet på en stationær computer på præcis samme relative placering, når den ses på en tablet eller telefon. Brugere kan samarbejde på tværs af enheder uden positioneringsinkonsistenser.

Dette virker tovejssikkert. En diskussion oprettet ved at trykke på et specifikt sted på en mobil enhed vises på samme relative position, når den ses på en stor desktopskærm.

### Viewport Considerations

Widgetten beregner procenter relativt til selve billedelementet, ikke visningsområdet. Det betyder, at scrolling af siden eller ændring af browservinduesstørrelse ikke påvirker markørpositionerne. Markørerne forbliver forankrede til deres placeringer på billedet uanset ændringer i visningsområdet.

### Future-Proofing Content

Den procentbaserede tilgang gør dine billeddiskussioner robuste over for ændringer i layout, design eller enhedsøkosystemet. Efterhånden som nye skærmstørrelser og enheder dukker op, vil de eksisterende diskussioner fortsætte med at blive vist korrekt uden at kræve opdateringer eller migreringer.