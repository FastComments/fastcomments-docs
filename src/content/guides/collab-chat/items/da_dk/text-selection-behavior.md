### Sådan fungerer tekstmarkering

Når brugere markerer tekst i Collab Chat-containeren, fanger widgetten markeringen og giver dem mulighed for at starte en diskussion. Markeringen kan være så lille som et enkelt ord eller så stor som flere afsnit, der spænder over forskellige elementer.

### Markeringsforsinkelse

På stationære enheder er der en forsinkelse på 3,5 sekunder mellem, hvornår en bruger markerer tekst, og hvornår diskussionsprompten vises. Dette forhindrer, at brugerfladen flimrer, når brugere blot markerer tekst for at kopiere eller læse den. På mobile enheder vises prompten med det samme, da tekstmarkering er mere bevidst på touchskærme.

### Unikke samtale-ID'er

Hver samtale får et unikt `urlId`, der kombinerer sidens URL, DOM-elementstien og det serialiserede tekstområde. Dette sikrer, at hver tekstmarkering skaber en distinkt samtale, som kan findes igen senere.

Hvis du angiver et brugerdefineret `urlId` i din konfiguration, vil det blive kombineret med elementstien og tekstområdet for at danne den endelige identifikator.

### Visuelle fremhævninger

Når der findes en diskussion for en bestemt tekstmarkering, får teksten en visuel fremhævning. Fremhævningen implementeres med baggrundsfarver og vises ved hover, eller når det tilknyttede chatvindue er åbent.

Fremhævningssystemet fungerer ved at indpakke den markerede tekst i et specielt element, som kan styles. Denne tilgang sikrer, at fremhævninger forbliver præcise, selv når den underliggende HTML-struktur er kompleks.

### Placering af chatvinduer

Når en bruger klikker på en fremhævning eller opretter en ny annotering, vises et chatvindue nær den markerede tekst. Widgetten beregner automatisk den bedste placering for dette vindue baseret på den tilgængelige plads i viewporten.

Positioneringssystemet bruger CSS-klasser som `to-right`, `to-left`, `to-top` og `to-bottom` til at angive, i hvilken retning chatvinduet skal strække sig fra fremhævningen. På mobile enheder (skærme under 768px i bredden) vises chatvinduet altid i fuldskærm for bedre brugervenlighed.

### Chatvinduets dimensioner

Chatvinduer er 410px brede på stationære enheder med 20px afstand og en 16px visuel pil, der peger på den fremhævede tekst. Disse dimensioner er faste for at sikre konsistent adfærd, men du kan tilpasse udseendet med CSS.

### Markeringer på tværs af elementer

Brugere kan markere tekst, der spænder over flere HTML-elementer, f.eks. ved at fremhæve fra midten af ét afsnit til begyndelsen af et andet. Range-serialiseringssystemet håndterer dette korrekt og vil fremhæve hele den markerede tekst, selv på tværs af elementgrænser.

### Browserkompatibilitet

Tekstmarkeringssystemet bruger standard-API'en `window.getSelection()`, som understøttes i alle moderne browsere. For ældre versioner af Internet Explorer falder det tilbage til `document.selection` af hensyn til kompatibiliteten.

### Vedvarende markeringer

Når en samtale er oprettet for en tekstmarkering, forbliver annoteringen, selv hvis siden genindlæses. Det serialiserede område og DOM-stien gør det muligt for widgetten at gendanne fremhævninger på præcis samme sted, når brugerne vender tilbage til siden.

Dette fungerer pålideligt, så længe dit sideindhold forbliver stabilt. Hvis du ændrer tekstindholdet eller omstrukturerer din HTML, vil eksisterende annoteringer muligvis ikke længere passe korrekt til teksten. Af denne grund er det bedst at undgå større indholdsændringer på sider med aktive annoteringer, eller overveje at migrere annoteringer, når indholdsændringer er nødvendige.