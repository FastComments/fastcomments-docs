**Skabelon-ID:** `welcome_greeter`

Welcome Greeter svarer varmt på førstegangs-kommentatorer. Det er den lavrisiko-skabelon (ingen destruktive værktøjer) og en god første agent at sætte i produktion.

### Udløsere

- **Ny bruger poster sin første kommentar på dette site** (`NEW_USER_FIRST_COMMENT`).

Denne hændelse affyres præcis én gang per bruger, så agenten kan ikke gå i loop. Se [Udløser: Ny brugers første kommentar](#trigger-new-user-first-comment).

### Tilladte værktøjer

- [`write_comment`](#tools-overview)

Det er det eneste værktøj - agenten kan bogstaveligt talt ikke moderere, stemme, bannlyse eller sende private beskeder (DM).

### Anbefalede tilføjelser inden live-sætning

- **Sæt Visningsnavnet** til noget indbydende - "Fællesskabsbot", jeres site-maskot eller jeres brandnavn. Visningsnavnet er det, læsere ser knyttet til velkomstsvaret.
- **Sæt flueben ved "Inkluder sidens titel, undertitel, beskrivelse og meta-tags"** i [Kontekstindstillinger](#context-options). Greeterens svar bliver mærkbart bedre, når den kan referere til, hvad siden faktisk handler om.
- **Overvej lokalitetsbegrænsninger** hvis I opererer på flere sprog. Et velkomstsvar på det forkerte sprog er mere forstyrrende end et manglende svar. Se [Omfang: URL- og sprogfiltre](#scope-url-locale).

### Hvorfor der ikke er behov for godkendelser

Agenten skriver kun nye kommentarer og kun på en engangsudløser. I værste fald: en akavet hilsen. Der er ingen destruktiv handling at spærrre for. De fleste operatører kører denne uden godkendelser, når testkørslen ser ren ud.

---