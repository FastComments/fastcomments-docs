### Realtime-updates

Collab Chat gebruikt WebSocket-verbindingen om alle gesprekken in realtime te synchroniseren tussen alle verbonden gebruikers. Wanneer iemand een nieuwe annotatie maakt, een opmerking toevoegt of een discussie verwijdert, zien alle andere gebruikers die dezelfde pagina bekijken de update onmiddellijk zonder te verversen.

### Hoe WebSocket-synchronisatie werkt

Wanneer je Collab Chat initialiseert, maakt de widget een WebSocket-verbinding met de FastComments-servers. Deze verbinding blijft open gedurende de sessie van de gebruiker en luistert naar updates die betrekking hebben op de huidige pagina.

Het WebSocket-systeem gebruikt drie typen broadcast-berichten voor Collab Chat. Het `new-text-chat`-event wordt geactiveerd wanneer iemand een nieuwe annotatie op de pagina maakt. Het `updated-text-chat`-event wordt geactiveerd wanneer iemand een bestaand gesprek bijwerkt. Het `deleted-text-chat`-event wordt geactiveerd wanneer iemand een annotatie verwijdert.

### Broadcast-ID-systeem

Om echo-effecten te voorkomen waarbij gebruikers hun eigen acties teruggezien krijgen, bevat elke update een unieke `broadcastId`. Wanneer een gebruiker een annotatie maakt of bijwerkt, genereert hun client een UUID voor die bewerking. Wanneer de WebSocket de update terug naar alle clients uitzendt, negeert de oorspronkelijke client de update omdat deze overeenkomt met zijn eigen `broadcastId`.

Dit zorgt voor een vloeiende interactie waarbij gebruikers hun wijzigingen onmiddellijk in de UI zien zonder te wachten op de retourronde via de server, terwijl alsnog wordt gegarandeerd dat alle andere gebruikers de update ontvangen.

### Live gebruikersaantal

De bovenste balk toont het aantal gebruikers dat momenteel de pagina bekijkt. Dit aantal wordt in realtime bijgewerkt wanneer gebruikers toetreden en vertrekken. Het gebruikersaantal wordt geleverd via dezelfde WebSocket-verbinding en neemt automatisch toe/af op basis van verbindings- en ontkoppelingsgebeurtenissen.

### Verbindingsherstel

Als de WebSocket-verbinding wegvalt door netwerkproblemen of serveronderhoud, probeert de widget automatisch opnieuw verbinding te maken. Tijdens de periode van opnieuw verbinden kunnen gebruikers nog steeds met bestaande annotaties werken, maar ze zien geen realtime-updates van andere gebruikers totdat de verbinding is hersteld.

Zodra de verbinding opnieuw tot stand is gebracht, synchroniseert de widget opnieuw om ervoor te zorgen dat er geen updates zijn gemist. Dit gebeurt transparant zonder dat de gebruiker iets hoeft te doen.

### Bandbreedte-overwegingen

WebSocket-berichten zijn lichtgewicht en bevatten alleen de essentiële informatie die nodig is om de staat te synchroniseren. Het aanmaken van een nieuwe annotatie gebruikt doorgaans minder dan 1KB aan bandbreedte. Het systeem bevat ook intelligente batching om de frequentie van berichten te verminderen tijdens periodes met veel activiteit.

Je gebruiksstatistieken in het FastComments-dashboard volgen `pubSubMessageCount` en `pubSubBandwidth` zodat je de realtime-synchronisatie-activiteit over je sites kunt monitoren.

### Synchronisatie tussen tabbladen

Als een gebruiker dezelfde pagina in meerdere browsertabbladen geopend heeft, verschijnen updates in het ene tabblad direct in de andere tabbladen. Dit werkt via hetzelfde WebSocket-synchronisatiemechanisme en vereist geen extra configuratie.

### Beveiliging

WebSocket-berichten worden verzonden over beveiligde verbindingen (WSS) en bevatten tenant-validatie om ervoor te zorgen dat gebruikers alleen updates ontvangen voor gesprekken waarvoor ze geautoriseerd zijn. De server valideert alle bewerkingen voordat deze naar voren worden uitgezonden om ongeautoriseerde toegang of manipulatie te voorkomen.