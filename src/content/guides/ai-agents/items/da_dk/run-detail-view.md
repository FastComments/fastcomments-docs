Klik på **Vis** på en række i [Kørselshistorik](#run-history) åbner detaljesiden for den enkelte kørsel. Her kan du læse agentens ræsonnement og bedømme dens beslutninger.

### Øverst: kørselssammendrag

- **Agent** - hvilken agent kørte.
- **Hvornår** - tidsstempel.
- **Status** - Startet / Succes / Fejl, plus **Tør kørsel**-badge hvis relevant.
- **Cost** - omkostning per kørsel i din tenant's valuta.
- **Cost per action** - omkostningen delt med antallet af ikke-ventende handlinger, nyttig til at opdage usædvanligt dyre kørsel.

### Udførte handlinger

En liste, i rækkefølge, af hvert værktøjskald som kørslen foretog. Hvert indslag viser:

- **Action label** - "Skriv en kommentar", "Markerede en kommentar som spam", "Bannede en bruger", og så videre. Etiketten mappes fra enum'en for handlingstyper.
- **Reference ID** - den berørte kommentar-, bruger- eller badge-ID, vist som monospaced tekst (ikke et hyperlink).
- **Agent reasoning** - den begrundelse agenten gav med kaldet.
- **Confidence** - agentens selvvurderede konfidens, vist som en procentdel.
- **Pending approval** badge - hvis handlingen er sat i kø i [godkendelsesindbakken](#approval-workflow) i stedet for at blive udført.

Hvis kørslen ikke udførte nogen handlinger, står der i sektionen: "Ingen handlinger blev udført under denne kørsel."

### LLM-transkript

Under handlingerne følger hele transskriptet af agentens samtale med LLM'en:

- **System** - systemprompten (platformsuffiks + din indledende prompt + fællesskabets retningslinjer).
- **User** - kontekstbeskeden der beskriver udløseren.
- **Assistant** - modellens svar, inklusive værktøjskald.
- **Tool** - værktøjsresultatet der blev ført tilbage til modellen (f.eks. hvad `search_memory` returnerede).

Lange beskeder kan foldes sammen; klik **Udvid** / **Skjul** for at se.

### Læsning af transskripter

Transskriptet er den vigtigste side til tuning. Når agenten træffer en beslutning, du er uenig i, læs det tilbage for at se:

- Hvad modellen **så** (Brugerens kontekstbesked).
- Hvad modellen **besluttede** (Assistentens værktøjskald).
- Hvad modellen **overvejede** (eventuelle værktøjsresultater - f.eks. kaldte agenten faktisk `search_memory` og fandt den noget, før den bannede).

Hvis modellen konsekvent begår samme slags fejl, rediger [indledende prompt](#personality-prompt) - eller brug [Forfining af prompts](#refining-prompts) fra en afvist godkendelse.

### Handlingsreferencer

Reference-ID'erne vises som monospaced tekst (ikke hyperlinks):

- Kommentarer: kommentarens ID.
- Brugere: bruger-ID'et.
- Badges: badge-ID'et.

Du kan kopiere ID'et for at slå den berørte post op på den relevante moderations-/adminside.

### Hvad mangler i Tør kørsel

Tørkørsler viser de **samme** handlinger, begrundelser og konfidensscore. Den eneste forskel er **Tør kørsel**-badge'en på statusraden. Reference-ID'erne for kommentarer / brugere / badges vises stadig - agenten påvirkede dem bare ikke.

### Fejl

For kørsler i `Error`-tilstand viser detaljesiden den underliggende fejlmeddelelse. Almindelige fejl:

- **No LLM API key configured** - tenant- eller platformfejlkonstruktion.
- **LLM call timeout** - LLM-udbyderen var langsom eller utilgængelig.
- **Tool dispatch failure** - agenten valgte et værktøj med dårlige argumenter (f.eks. et kommentar-ID, der ikke længere findes).
- **Budget exhausted mid-run** - agentens grænse blev ramt mens kørslen var i gang. Kørslen blev standset.

Fejl ruller ikke delvise handlinger tilbage - alle værktøjskald, der blev fuldført før fejlen, forbliver.