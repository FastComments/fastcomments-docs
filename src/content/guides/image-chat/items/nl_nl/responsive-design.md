### Op percentages gebaseerde positionering

Image Chat gebruikt op percentages gebaseerde coördinaten in plaats van pixelcoördinaten om chatmarkeringen op afbeeldingen te positioneren. Wanneer een gebruiker op een afbeelding klikt, zet de widget de pixelcoördinaten van de klik om in percentages van de afbeeldingsbreedte en -hoogte. Deze aanpak zorgt ervoor dat markeringen op de juiste plaats blijven, ongeacht hoe de afbeelding wordt weergegeven.

Als voorbeeld: als een gebruiker 250 pixels vanaf de linkerrand van een afbeelding van 1000px breed klikt, slaat de widget dit op als 25% vanaf links. Wanneer een andere gebruiker dezelfde afbeelding op een mobiel apparaat op 500px breed bekijkt, verschijnt de markering op 125 pixels vanaf links, wat nog steeds 25% van de breedte is.

### Voordelen voor responsieve lay-outs

Dit percentagesysteem zorgt ervoor dat Image Chat naadloos werkt op alle apparaatsgroottes en -oriëntaties. Uw afbeeldingen kunnen afhankelijk van schermbreedte, CSS-regels of containerbeperkingen in verschillende formaten worden weergegeven, maar de markeringen blijven altijd correct uitgelijnd met de bedoelde locaties.

Gebruikers op desktopcomputers met grote monitoren zien markeringen op dezelfde relatieve posities als gebruikers op smartphones met kleine schermen. De markeringen schalen evenredig mee met de afbeelding zelf.

### Schaal en beeldverhouding van afbeeldingen

Zolang uw afbeelding zijn beeldverhouding behoudt tijdens het schalen (wat het standaardgedrag van browsers is), blijven de op percentages gebaseerde markeringen perfect uitgelijnd. De widget gaat ervan uit dat afbeeldingen proportioneel schalen en berekent posities op basis van die aanname.

Als u CSS toepast die de beeldverhouding van de afbeelding vervormt (zoals het gebruik van `object-fit: cover` met specifieke afmetingen), kunnen de markeringen mogelijk niet correct uitgelijnd zijn. Voor de beste resultaten laat u afbeeldingen natuurlijk schalen of gebruikt u `object-fit: contain` om de beeldverhouding te behouden.

### Grootte van chatvierkanten

Het visuele formaat van chatmarkeringen is ook op percentages gebaseerd. De configuratieoptie `chatSquarePercentage` staat standaard op 5%, wat betekent dat elk vierkant 5% van de afbeeldingsbreedte is. Dit zorgt voor een consistente visuele weging op verschillende afbeeldingsgroottes.

Op een afbeelding van 1000px breed met de standaardinstelling van 5% zijn markeringen 50px vierkant. Op een afbeelding van 500px breed zijn dezelfde markeringen 25px vierkant. Ze blijven evenredig met de afbeelding.

### Gedrag op mobiel

Op schermen die smaller zijn dan 768px schakelt Image Chat over naar een mobiel-geoptimaliseerde lay-out. Chatvensters openen in volledig scherm in plaats van zwevend naast de markering. Dit zorgt voor betere bruikbaarheid op kleine schermen waar zwevende vensters te veel van de afbeelding zouden bedekken.

De markeringen zelf blijven zichtbaar en klikbaar op hun op percentages gebaseerde posities. Gebruikers kunnen nog steeds zien waar alle discussies zich bevinden en op markeringen tikken om de volwaardige chatinterface in volledig scherm te openen.

### Dynamisch laden van afbeeldingen

Het op percentages gebaseerde systeem werkt correct, zelfs wanneer afbeeldingen asynchroon laden of van grootte veranderen nadat de pagina is geladen. De widget bewaakt het afbeeldings-element en herberekent markeerposities wanneer de afbeeldingsdimensies veranderen.

Als u afbeeldingen lazy-loadt of responsieve afbeeldingen implementeert met verschillende groottes bij verschillende breakpoints, passen de markeringen zich automatisch aan wanneer de afbeeldingsgrootte verandert.

### Consistentie tussen apparaten

Omdat coördinaten als percentages in de database worden opgeslagen, verschijnt een discussie die op een desktopcomputer is aangemaakt op exact dezelfde relatieve locatie wanneer deze op een tablet of telefoon wordt bekeken. Gebruikers kunnen samenwerken tussen apparaten zonder positionele inconsistenties.

Dit werkt in twee richtingen. Een discussie die is aangemaakt door op een specifieke plek op een mobiel apparaat te tikken, verschijnt op dezelfde relatieve positie wanneer deze op een grote desktopmonitor wordt bekeken.

### Viewport-overwegingen

De widget berekent percentages relatief ten opzichte van het afbeeldings-element zelf, niet ten opzichte van de viewport. Dit betekent dat scrollen op de pagina of het veranderen van de grootte van het browservenster geen invloed heeft op markeerposities. Markeringen blijven verankerd op hun locaties op de afbeelding ongeacht wijzigingen in de viewport.

### Toekomstbestendigheid van content

De op percentages gebaseerde aanpak maakt uw afbeeldingsdiscussies bestand tegen veranderingen in lay-out, ontwerp of apparaatecosysteem. Naarmate nieuwe schermformaten en apparaten verschijnen, blijven bestaande discussies correct worden weergegeven zonder dat updates of migraties nodig zijn.