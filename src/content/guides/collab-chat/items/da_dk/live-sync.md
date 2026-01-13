### Opdateringer i realtid

Collab Chat bruger WebSocket-forbindelser til at synkronisere alle samtaler i realtid på tværs af alle tilsluttede brugere. Når nogen opretter en ny annotering, tilføjer en kommentar eller sletter en diskussion, ser alle andre brugere, der ser samme side, opdateringen med det samme uden at genindlæse.

### Hvordan WebSocket-synkronisering fungerer

Når du initialiserer Collab Chat, etablerer widgeten en WebSocket-forbindelse til FastComments-serverne. Denne forbindelse forbliver åben i brugerens session og lytter efter opdateringer relateret til den aktuelle side.

WebSocket-systemet bruger tre typer broadcast-beskeder for Collab Chat. `new-text-chat`-begivenheden udløses, når nogen opretter en ny annotering på siden. `updated-text-chat`-begivenheden udløses, når nogen opdaterer en eksisterende samtale. `deleted-text-chat`-begivenheden udløses, når nogen sletter en annotering.

### Broadcast ID-system

For at forhindre ekko-effekter, hvor brugere ser deres egne handlinger blive broadcastet tilbage til dem, indeholder hver opdatering en unik `broadcastId`. Når en bruger opretter eller opdaterer en annotering, genererer deres klient en UUID for den handling. Når WebSocket'en broadcaster opdateringen tilbage til alle klienter, ignorerer den oprindelige klient opdateringen, fordi den matcher dens eget `broadcastId`.

Dette sikrer en glidende interaktion, hvor brugere ser deres ændringer med det samme i brugergrænsefladen uden at vente på tur/retur gennem serveren, samtidig med at alle andre brugere stadig modtager opdateringen.

### Live-brugerantal

Topbaren viser antallet af brugere, der i øjeblikket ser siden. Dette antal opdateres i realtid, efterhånden som brugere slutter sig til og forlader siden. Brugerantallet leveres gennem den samme WebSocket-forbindelse og øges/formindskes automatisk baseret på forbindelses- og afbrydelsesbegivenheder.

### Forbindelsesstabilitet

Hvis WebSocket-forbindelsen falder på grund af netværksproblemer eller servervedligeholdelse, forsøger widgeten automatisk at genoprette forbindelsen. I løbet af genoprettelsesperioden kan brugere stadig interagere med eksisterende annoteringer, men de vil ikke se realtidsopdateringer fra andre brugere, indtil forbindelsen er genoprettet.

Når forbindelsen er genoprettet, resynkroniserer widgeten for at sikre, at ingen opdateringer gik tabt. Dette sker transparent uden at kræve brugerintervention.

### Båndbreddehensyn

WebSocket-beskeder er lette og indeholder kun de væsentlige oplysninger, der er nødvendige for at synkronisere tilstanden. Oprettelse af en ny annotering bruger typisk mindre end 1KB båndbredde. Systemet inkluderer også intelligent batching for at reducere beskedfrekvensen under perioder med høj aktivitet.

Dine brugermetrikker i FastComments-dashboardet sporer `pubSubMessageCount` og `pubSubBandwidth`, så du kan overvåge realtids-synkroniseringsaktivitet på tværs af dine sites.

### Synkronisering på tværs af faner

Hvis en bruger har samme side åben i flere faner i browseren, vises opdateringer i én fane med det samme i de andre faner. Dette fungerer gennem den samme WebSocket-synkroniseringsmekanisme og kræver ingen ekstra konfiguration.

### Sikkerhed

WebSocket-beskeder transmitters over sikre forbindelser (WSS) og inkluderer tenant-validering for at sikre, at brugere kun modtager opdateringer for samtaler, de har tilladelse til at se. Serveren validerer alle operationer, før de broadcastes, for at forhindre uautoriseret adgang eller manipulation.