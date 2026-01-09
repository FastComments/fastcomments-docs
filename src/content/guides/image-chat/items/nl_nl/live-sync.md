### Realtime-updates

Image Chat gebruikt WebSocket-verbindingen om alle gesprekken in realtime te synchroniseren voor alle verbonden gebruikers. Wanneer iemand een nieuwe marker maakt, een opmerking toevoegt of een discussie verwijdert, zien alle andere gebruikers die dezelfde afbeelding bekijken de update onmiddellijk zonder te vernieuwen.

### Hoe WebSocket-synchronisatie werkt

Wanneer je Image Chat initialiseert, maakt de widget een WebSocket-verbinding met de FastComments-servers. Deze verbinding blijft open voor de duur van de gebruikerssessie en luistert naar updates die betrekking hebben op de huidige afbeelding.

Het WebSocket-systeem gebruikt drie typen broadcast-berichten voor Image Chat. Het `new-image-chat` event wordt afgevuurd wanneer iemand een nieuwe marker op de afbeelding aanmaakt. Het `image-chat-updated` event wordt afgevuurd wanneer iemand een bestaand gesprek bijwerkt. Het `deleted-image-chat` event wordt afgevuurd wanneer iemand een marker verwijdert.

### Broadcast-ID-systeem

Om echo-effecten te voorkomen waarbij gebruikers hun eigen acties teruggezien krijgen, bevat elke update een unieke `broadcastId`. Wanneer een gebruiker een marker aanmaakt of bijwerkt, genereert hun client een UUID voor die bewerking. Wanneer de WebSocket de update terug uitzendt naar alle clients, negeert de oorspronkelijke client de update omdat deze overeenkomt met zijn eigen `broadcastId`.

Dit zorgt voor een soepele interactie waarbij gebruikers hun wijzigingen onmiddellijk in de UI zien zonder te wachten op de roundtrip via de server, terwijl alle andere gebruikers toch de update ontvangen.

### Verbindingsherstel

Als de WebSocket-verbinding wegvalt vanwege netwerkproblemen of serveronderhoud, probeert de widget automatisch opnieuw verbinding te maken. Tijdens de periode van opnieuw verbinden kunnen gebruikers nog steeds met bestaande markers interageren, maar ze zien geen realtime-updates van andere gebruikers totdat de verbinding is hersteld.

Zodra de verbinding is hersteld, synchroniseert de widget opnieuw om zeker te zijn dat er geen updates zijn gemist. Dit gebeurt transparant zonder dat de gebruiker iets hoeft te doen.

### Bandbreedte-overwegingen

WebSocket-berichten zijn lichtgewicht en bevatten alleen de essentiële informatie die nodig is om de status te synchroniseren. Het aanmaken van een nieuwe marker gebruikt doorgaans minder dan 1KB aan bandbreedte. Het systeem bevat ook intelligente batching om de frequentie van berichten te verminderen tijdens periodes met veel activiteit.

Je gebruiksstatistieken in het FastComments dashboard volgen `pubSubMessageCount` en `pubSubBandwidth` zodat je de realtime-synchronisatie-activiteit over je sites kunt monitoren.

### Synchronisatie tussen tabbladen

Als een gebruiker dezelfde pagina in meerdere browsertabbladen open heeft, verschijnen updates in het ene tabblad onmiddellijk in de andere tabbladen. Dit werkt via hetzelfde WebSocket-synchronisatiemechanisme en vereist geen extra configuratie.

Gebruikers kunnen je site ook op meerdere apparaten tegelijk open hebben en al deze apparaten blijven synchroon. Een marker die op een desktopcomputer is gemaakt, verschijnt direct op de tablet van de gebruiker als beide apparaten dezelfde afbeelding bekijken.

### Beveiliging

WebSocket-berichten worden verzonden via beveiligde verbindingen (WSS) en bevatten tenant-validatie om ervoor te zorgen dat gebruikers alleen updates ontvangen voor gesprekken waarvoor ze geautoriseerd zijn. De server valideert alle bewerkingen voordat deze worden uitgezonden om ongeautoriseerde toegang of manipulatie te voorkomen.

### Offline-gedrag

Wanneer gebruikers volledig offline zijn, kunnen ze bestaande markers nog bekijken maar kunnen ze geen nieuwe aanmaken of updates van anderen zien. De widget detecteert de offline-status en toont gepaste meldingen.

Als een gebruiker probeert een marker aan te maken terwijl hij offline is en daarna weer online komt, zal de bewerking mislukken in plaats van in de wachtrij te worden geplaatst, om gegevensconsistentie te waarborgen. Gebruikers moeten de bewerking opnieuw proberen zodra hun verbinding is hersteld.

### Impact op prestaties

De WebSocket-verbinding heeft een minimale impact op de prestaties. De verbinding blijft inactief wanneer er geen updates plaatsvinden en verwerkt alleen berichten wanneer er activiteit is. Bij een typische afbeelding met matige marker-activiteit gebruikt de WebSocket minder CPU dan het renderen van de afbeelding zelf.

Voor pagina's met honderden gelijktijdige gebruikers en veel marker-aanmaakactiviteit schaalt het systeem horizontaal om de prestaties te behouden zonder individuele clientverbindingen te beïnvloeden.

### Samenwerkingsscenario's

De realtime-synchronisatie maakt Image Chat bijzonder krachtig voor samenwerkingsworkflows. Ontwerpteams kunnen samen mockups beoordelen waarbij iedereen in realtime ziet waar markers worden geplaatst. Klantenserviceteams kunnen gezamenlijk screenshots annoteren om problemen te identificeren. Onderwijsgroepen kunnen diagrammen bespreken waarbij alle deelnemers elkaars markers zien terwijl ze worden gemaakt.

De directe feedback zorgt voor een meer betrokken en productieve samenwerkingservaring in vergelijking met traditionele reactiesystemen waarbij gebruikers moeten verversen om updates te zien.