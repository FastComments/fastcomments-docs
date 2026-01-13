### How Text Selection Works

Når brugere vælger tekst inden for Collab Chat containeren, fanger widgeten det valg og giver dem mulighed for at starte en diskussion. Valget kan være så lille som et enkelt ord eller så stort som flere afsnit, der går på tværs af forskellige elementer.

### Selection Delay

På stationære enheder er der en forsinkelse på 3,5 sekunder mellem tidspunktet, hvor en bruger vælger tekst, og hvor diskussionsprompten vises. Dette forhindrer, at brugergrænsefladen (UI) blinker, når brugere blot markerer tekst for at kopiere eller læse. På mobile enheder vises prompten øjeblikkeligt, da tekstvalg er mere bevidst på touchskærme.


### Unique Conversation IDs

Hver samtale får et unikt `urlId`, der kombinerer side-URL'en, DOM-elementstien og det serialiserede tekstrækkevidde. Dette sikrer, at hvert tekstvalg opretter en særskilt samtale, som kan findes igen senere.

Hvis du angiver et brugerdefineret `urlId` i din konfiguration, kombineres det med elementstien og tekstrækkevidden for at skabe den endelige identifikator.

### Visual Highlights

Når der findes en diskussion for et bestemt tekstvalg, får den tekst en visuel fremhævning. Fremhævningen er implementeret ved hjælp af baggrundsfarver og vises ved museover eller når det tilknyttede chatvindue er åbent.

Fremhævningssystemet fungerer ved at indpakke den valgte tekst i et specielt element, der kan styles. Denne tilgang sikrer, at fremhævninger forbliver nøjagtige, selv når den underliggende HTML-struktur er kompleks.

### Chat Window Positioning

Når en bruger klikker på en fremhævning eller opretter en ny annotation, vises et chatvindue nær den valgte tekst. Widgeten beregner automatisk den bedste position for dette vindue baseret på tilgængelig visningsplads.

Positionssystemet bruger CSS-klasser som `to-right`, `to-left`, `to-top` og `to-bottom` til at angive, i hvilken retning chatvinduet skal udvides fra fremhævningen. På mobile enheder (skærme under 768px i bredden) vises chatvinduet altid i fuldskærm for bedre brugervenlighed.

### Chat Window Dimensions

Chatvinduer er 410px brede på desktop med 20px afstand og en 16px visuel pil, der peger på den fremhævede tekst. Disse dimensioner er faste for at sikre konsekvent adfærd, men du kan tilpasse udseendet med CSS.

### Cross-Element Selections

Brugere kan vælge tekst, der spænder over flere HTML-elementer, såsom at markere fra midten af et afsnit til starten af et andet. Range-serialiseringssystemet håndterer dette korrekt og vil fremhæve al den valgte tekst, selv på tværs af elementgrænser.

### Browser Compatibility

Tekstudvælgelsessystemet bruger den standard `window.getSelection()` API, som understøttes i alle moderne browsere. For ældre versioner af Internet Explorer falder det tilbage til `document.selection` for kompatibilitet.

### Selection Persistence

Når en samtale er oprettet for et tekstvalg, bevares den annotation, selv hvis siden genindlæses. Det serialiserede rækkevidde og DOM-stien gør det muligt for widgeten at gendanne fremhævninger på præcis samme sted, når brugere vender tilbage til siden.

Dette fungerer pålideligt, så længe dit sideindhold forbliver stabilt. Hvis du ændrer tekstindholdet eller omstrukturerer din HTML, kan eksisterende annotationer muligvis ikke længere være korrekt tilpasset teksten. Af denne grund er det bedst at undgå større indholdsændringer på sider med aktive annotationer eller overveje at migrere annotationer, når indholdsændringer er nødvendige.