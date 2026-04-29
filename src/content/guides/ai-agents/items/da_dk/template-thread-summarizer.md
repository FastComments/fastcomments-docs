**Skabelon-ID:** `thread_summarizer`

Trådopsummereren lægger et neutralt, ét-afsnits resumé i slutningen af en lang tråd. Den bruger en 30-minutters forsinkelse, så tråden kan falde til ro, før agenten gennemgår den.

Den indbyggede prompt instruerer agenten i ikke at formulere subjektive vurderinger - dette er afgørende. Uden den har modellen tendens til at bruge formuleringer som "in my view", hvilket ikke ser godt ud under kontoens visningsnavn.

### Udløsere

- **Ny kommentar oprettet** (`COMMENT_ADD`).
- **Udløserforsinkelse**: 30 minutter (1800 sekunder). Se [Udskudte udløsere](#trigger-deferred-delay).

30-minutters forsinkelsen betyder, at agenten kører én gang, en halv time efter en kommentar lander, imod hvordan tråden ser ud i det øjeblik. Det er ikke "opsummér ved hvert svar" - køen for udskudte udløsere samler flere nye-kommentar-hændelser i samme tråd, men de-deuplicerer dem ikke på tværs af separate vinduer. Du vil sandsynligvis ønske at **tilføje en brugerdefineret regel i din prompt** som "læg ikke et nyt resumé, hvis agenten allerede har opsummeret denne tråd inden for de sidste 24 timer" og stole på kontekst plus agentens [Værktøjsoversigt](#tools-overview) for at håndhæve det.

### Tilladte værktøjer

- [`write_comment`](#tools-overview) - poster selve resuméet.
- [`pin_comment`](#tools-overview) - fastgør resuméet, så læsere ser det øverst i tråden.
- [`unpin_comment`](#tools-overview) - fjerner fastgørelsen af et tidligere resumé fra samme agent, før det nye fastgøres.

Trådopsummereren kan ikke moderere eller interagere med brugere.

### Fastgørelse af resuméet

Agenten poster en ny kommentar med `write_comment`, og kalder derefter `pin_comment` med det returnerede kommentar-id. Ved efterfølgende kørsel mod samme tråd instruerer prompten den i først at kalde `unpin_comment` på sit tidligere resumé - platformen håndhæver ikke en regel om kun én fastgjort kommentar pr. tråd, så hvis det tidligere resumé bliver efterladt fastgjort, vil det resultere i to fastgjorte resuméer side om side. Sæt kryds i "Inkluder overordnet kommentar og tidligere svar i samme tråd" under [Kontekstindstillinger](#context-options), så agenten kan se det tidligere fastgjorte resumé.

### Anbefalede tilføjelser før du går live

- **Sæt kryds i "Inkluder overordnet kommentar og tidligere svar i samme tråd"** under [Kontekstindstillinger](#context-options). En opsummerer uden trådkontekst er ubrugelig.
- **Juster reglen for minimums-trådstørrelse.** "Færre end 5 kommentarer" er promptens standard, men i travle fællesskaber er 10–20 mere passende. Rediger prompten direkte.
- **Begræns til specifikke URL-mønstre** hvis du kun vil have resuméer på langt-formede sider, ikke annonceringer eller produktsider. Se [Omfang: URL- og lokalitetsfiltre](#scope-url-locale).
- **Hold øje med omkostningerne.** Opsummering bruger flest tokens, fordi den læser hele tråden ved hver kørsel. Sæt et stramt [Budgetoversigt](#budgets-overview) før du slår til Enabled.

### Undgå gentagne resuméer

Agenten har adgang til [`save_memory`](#tools-overview) og [`search_memory`](#tools-overview) - du kan udvide prompten til at instruere den i at registrere noter som "summarized {thread urlId}" og tjekke for dem inden den poster igen. Hukommelsen deles på tværs af alle agenter i din tenant.