### Procentbaseret Positionering

Image Chat bruger procentbaserede koordinater i stedet for pixelkoordinater til at placere chatmarkører på billeder. Når en bruger klikker på et billede, konverterer widgetten klikkets pixelkoordinater til procenter af billedets bredde og højde. Denne tilgang sikrer, at markører forbliver på det korrekte sted uanset hvordan billedet vises.

For eksempel, hvis en bruger klikker 250 pixels fra venstre kant af et 1000px bredt billede, gemmer widgetten dette som 25% fra venstre. Når en anden bruger ser det samme billede 500px bredt på en mobil enhed, vises markøren 125 pixels fra venstre, hvilket stadig er 25% af bredden.

### Fordele ved Responsive Layouts

Dette procentsystem gør, at Image Chat fungerer problemfrit på alle enhedsstørrelser og orienteringer. Dine billeder kan vises i forskellige størrelser afhængigt af skærmbredde, CSS-regler eller containerbegrænsninger, men markørerne justeres altid korrekt i forhold til de tilsigtede placeringer.

Brugere på stationære computere med store skærme ser markørerne i de samme relative positioner som brugere på smartphones med små skærme. Markørerne skalerer proportionalt med selve billedet.

### Billedskalering og Aspektforhold

Så længe dit billede bevarer sit aspektforhold ved skalering (hvilket er browserens standardadfærd), forbliver de procentbaserede markører perfekt justeret. Widgetten antager, at billeder skalerer proportionalt, og beregner positioner ud fra denne antagelse.

Hvis du anvender CSS, der forvrider billedets aspektforhold (som at bruge `object-fit: cover` med specifikke dimensioner), stemmer markørpositionerne muligvis ikke overens. For de bedste resultater, lad billeder skalere naturligt eller brug `object-fit: contain` for at bevare aspektforholdet.

### Størrelse af Chatkvadrater

Den visuelle størrelse af chatmarkørerne er også procentbaseret. Konfigurationsindstillingen `chatSquarePercentage` er som standard 5%, hvilket betyder, at hvert kvadrat er 5% af billedets bredde. Dette sikrer en ensartet visuel vægt på tværs af forskellige billedstørrelser.

På et 1000px bredt billede med standardindstillingen på 5% er markørerne 50px i kvadrat. På et 500px bredt billede er de samme markører 25px i kvadrat. De forbliver proportionale med billedstørrelsen.

### Mobiladfærd

På skærme under 768px brede skifter Image Chat til et mobiloptimeret layout. Chatvinduer åbnes i fuldskærm i stedet for at flyde ved siden af markøren. Dette giver bedre brugervenlighed på små skærme, hvor flydende vinduer ville skjule for meget af billedet.

Markørerne selv forbliver synlige og klikbare på deres procentbaserede positioner. Brugere kan stadig se, hvor alle diskussioner er placeret, og trykke på markører for at åbne chatgrænsefladen i fuldskærm.

### Dynamisk Billedindlæsning

Det procentbaserede system fungerer korrekt, selv når billeder indlæses asynkront eller ændrer størrelse, efter at siden er indlæst. Widgetten overvåger billedelementet og genberegner markørpositioner, når billedets dimensioner ændres.

Hvis du lazy-loader billeder eller implementerer responsive billeder med forskellige størrelser ved forskellige breakpoints, justeres markørerne automatisk, når billedstørrelsen ændres.

### Konsistens på Tværs af Enheder

Fordi koordinaterne gemmes som procenter i databasen, vises en diskussion, der oprettes på en stationær computer, på præcis samme relative placering, når den ses på en tablet eller telefon. Brugere kan samarbejde på tværs af enheder uden nogen positioneringsinkonsistenser.

Dette fungerer i begge retninger. En diskussion, der oprettes ved at trykke på et bestemt sted på en mobil enhed, vises på samme relative position, når den ses på en stor stationær skærm.

### Betragtninger om Viewport

Widgetten beregner procenter i forhold til selve billedelementet, ikke viewporten. Det betyder, at rulning på siden eller ændring af browserens vinduesstørrelse ikke påvirker markørpositionerne. Markørerne forbliver forankret til deres placeringer på billedet uanset ændringer i viewporten.

### Fremtidssikring af Indhold

Den procentbaserede tilgang gør dine billedediskussioner modstandsdygtige over for ændringer i layout, design eller enhedsøkosystemet. Efterhånden som nye skærmstørrelser og enheder opstår, vil de eksisterende diskussioner fortsætte med at vises korrekt uden at kræve opdateringer eller migreringer.