---
### Hoe tekstselectie werkt

Wanneer gebruikers tekst selecteren binnen de Collab Chat-container, legt de widget die selectie vast en stelt hen in staat een discussie te starten. De selectie kan zo klein zijn als één woord of zo groot als meerdere alinea's die verschillende elementen overspannen.

### Selectievertraging

Op desktopapparaten is er een vertraging van 3,5 seconden tussen het moment dat een gebruiker tekst selecteert en het moment waarop de discussieprompt verschijnt. Dit voorkomt dat de UI knippert wanneer gebruikers gewoon tekst markeren om te kopiëren of te lezen. Op mobiele apparaten verschijnt de prompt onmiddellijk aangezien tekstselectie op aanraakschermen doelbewuster is.

### Unieke conversatie-ID's

Elke conversatie krijgt een unieke `urlId` die de paginakoppeling, het DOM-elementpad en het geserialiseerde textrange combineert. Dit zorgt ervoor dat elke tekstselectie een afzonderlijke conversatie creëert die later weer teruggevonden kan worden.

Als u een aangepaste `urlId` in uw configuratie opgeeft, wordt deze gecombineerd met het elementpad en het textrange om de uiteindelijke identifier te maken.

### Visuele markeringen

Wanneer er een discussie bestaat voor een bepaalde tekstselectie, krijgt die tekst een visuele markering. De markering wordt geïmplementeerd met behulp van achtergrondkleuren en verschijnt bij hover of wanneer het bijbehorende chatvenster geopend is.

Het markeersysteem werkt door de geselecteerde tekst te omhullen met een speciaal element dat gestyled kan worden. Deze aanpak zorgt ervoor dat markeringen nauwkeurig blijven, zelfs wanneer de onderliggende HTML-structuur complex is.

### Positionering van het chatvenster

Wanneer een gebruiker op een markering klikt of een nieuwe annotatie maakt, verschijnt er een chatvenster in de buurt van de geselecteerde tekst. De widget berekent automatisch de beste positie voor dit venster op basis van de beschikbare viewportruimte.

Het positioneringssysteem gebruikt CSS-klassen zoals `to-right`, `to-left`, `to-top`, en `to-bottom` om aan te geven in welke richting het chatvenster vanaf de markering moet uitvouwen. Op mobiele apparaten (schermen smaller dan 768px) verschijnt het chatvenster altijd fullscreen voor betere bruikbaarheid.

### Afmetingen van het chatvenster

Chatvensters zijn 410px breed op desktop met 20px tussenruimte en een visuele pijl van 16px die naar de gemarkeerde tekst wijst. Deze afmetingen zijn vast om consistent gedrag te garanderen, maar u kunt de uitstraling aanpassen met CSS.

### Selecties over meerdere elementen

Gebruikers kunnen tekst selecteren die meerdere HTML-elementen overspant, zoals markeren vanaf het midden van de ene alinea tot het begin van een andere. Het range-serialisatiesysteem behandelt dit correct en zal alle geselecteerde tekst markeren, zelfs over elementgrenzen heen.

### Browsercompatibiliteit

Het tekstselectiesysteem gebruikt de standaard `window.getSelection()` API die wordt ondersteund in alle moderne browsers. Voor oudere versies van Internet Explorer valt het terug op `document.selection` voor compatibiliteit.

### Persistentie van selecties

Zodra er een conversatie is aangemaakt voor een tekstselectie, blijft die annotatie bestaan zelfs als de pagina wordt vernieuwd. Het geserialiseerde range en het DOM-pad stellen de widget in staat markeringen op precies dezelfde plaats te herstellen wanneer gebruikers terugkeren naar de pagina.

Dit werkt betrouwbaar zolang uw paginacontent stabiel blijft. Als u de tekstinhoud wijzigt of uw HTML herstructureert, kunnen bestaande annotaties mogelijk niet meer correct uitlijnen met de tekst. Om deze reden is het het beste om grote inhoudswijzigingen op pagina's met actieve annotaties te vermijden, of te overwegen annotaties te migreren wanneer inhoudswijzigingen noodzakelijk zijn.

---