### Opdateringer i realtid

Image Chat bruger WebSocket-forbindelser til at synkronisere alle samtaler i realtid på tværs af alle tilsluttede brugere. Når nogen opretter en ny markør, tilføjer en kommentar eller sletter en diskussion, ser alle andre brugere, der ser det samme billede, opdateringen med det samme uden at opdatere siden.

### Hvordan WebSocket-synkronisering fungerer

Når du initialiserer Image Chat, etablerer widget'en en WebSocket-forbindelse til FastComments-serverne. Denne forbindelse forbliver åben i hele brugerens session og lytter efter opdateringer relateret til det aktuelle billede.

WebSocket-systemet bruger tre typer broadcast-beskeder til Image Chat. `new-image-chat`-begivenheden udløses, når nogen opretter en ny markør på billedet. `image-chat-updated`-begivenheden udløses, når nogen opdaterer en eksisterende samtale. `deleted-image-chat`-begivenheden udløses, når nogen sletter en markør.

### Broadcast ID-system

For at forhindre ekkoeffekter, hvor brugere ser deres egne handlinger blive sendt tilbage til dem, indeholder hver opdatering et unikt `broadcastId`. Når en bruger opretter eller opdaterer en markør, genererer deres klient en UUID til den handling. Når WebSocket'en udsender opdateringen tilbage til alle klienter, ignorerer den oprindelige klient opdateringen, fordi den matcher sit eget `broadcastId`.

Dette sikrer en glidende interaktion, hvor brugere ser deres ændringer med det samme i brugergrænsefladen uden at skulle vente på rundturen gennem serveren, samtidig med at alle andre brugere får opdateringen.

### Forbindelsesrobusthed

Hvis WebSocket-forbindelsen falder på grund af netværksproblemer eller servervedligeholdelse, forsøger widget'en automatisk at genoprette forbindelsen. I genoprettelsesperioden kan brugerne stadig interagere med eksisterende markører, men de vil ikke se realtidsopdateringer fra andre brugere, før forbindelsen er genoprettet.

Når forbindelsen er genoprettet, resynkroniserer widget'en for at sikre, at ingen opdateringer er gået tabt. Dette sker transparent uden at kræve brugerindgriben.

### Overvejelser om båndbredde

WebSocket-beskeder er lette og indeholder kun de nødvendige oplysninger til at synkronisere tilstanden. Oprettelse af en ny markør bruger typisk mindre end 1KB båndbredde. Systemet inkluderer også intelligent batching for at reducere beskedfrekvensen i perioder med høj aktivitet.

Dine brugermålinger i FastComments-dashboardet sporer `pubSubMessageCount` og `pubSubBandwidth`, så du kan overvåge realtids-synkroniseringsaktivitet på tværs af dine sites.

### Synkronisering på tværs af faner

Hvis en bruger har samme side åben i flere browserfaner, vises opdateringer i én fane med det samme i de andre faner. Dette fungerer gennem den samme WebSocket-synkroniseringsmekanisme og kræver ingen yderligere konfiguration.

Brugere kan have dit site åbent på flere enheder samtidigt, og alle vil forblive synkroniserede. En markør oprettet på en stationær computer vises øjeblikkeligt på brugerens tablet, hvis begge enheder ser det samme billede.

### Sikkerhed

WebSocket-beskeder sendes over sikre forbindelser (WSS) og inkluderer tenant-validering for at sikre, at brugere kun modtager opdateringer til samtaler, de er autoriserede til at se. Serveren validerer alle handlinger, før de udsendes, for at forhindre uautoriseret adgang eller manipulation.

### Offline-adfærd

Når brugere er helt offline, kan de stadig se eksisterende markører, men de kan ikke oprette nye eller se opdateringer fra andre. Widget'en registrerer offline-tilstanden og viser passende beskeder.

Hvis en bruger forsøger at oprette en markør, mens vedkommende er offline, og senere går online igen, vil handlingen fejle i stedet for at blive køet, for at sikre datakonsistens. Brugere bør prøve handlingen igen, når deres forbindelse er gendannet.

### Indvirkning på ydeevnen

WebSocket-forbindelsen har minimal indvirkning på ydeevnen. Forbindelsen forbliver inaktiv, når der ikke sker opdateringer, og behandler kun beskeder, når der er aktivitet. På et typisk billede med moderat markøraktivitet bruger WebSocket'en mindre CPU end selve gengivelsen af billedet.

For sider med hundredevis af samtidige brugere og høj aktivitet i oprettelse af markører skalerer systemet horisontalt for at opretholde ydeevnen uden at påvirke individuelle klientforbindelser.

### Samarbejdsbrugstilfælde

Realtidssynkroniseringen gør Image Chat særligt stærkt til samarbejdsarbejdsgange. Designteams kan gennemgå mockups sammen, hvor alle ser markørplaceringer i realtid. Kundesupportteams kan i fællesskab annotere skærmbilleder for at identificere problemer. Undervisningsgrupper kan diskutere diagrammer, hvor alle deltagere ser hinandens markører, efterhånden som de oprettes.

Den umiddelbare feedback skaber en mere engagerende og produktiv samarbejdsoplevelse sammenlignet med traditionelle kommentarsystemer, hvor brugere skal opdatere siden for at se opdateringer.