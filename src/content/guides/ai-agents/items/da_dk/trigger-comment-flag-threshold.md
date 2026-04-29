Udløses når en kommentar's flag-tæller når **præcis** den konfigurerede tærskel.

### Påkrævet konfiguration

- **Flag threshold** - heltal >= 1. Triggeren udløses i det øjeblik `flagCount === flagThreshold`. Den udløses ikke igen ved efterfølgende flags ud over tærsklen.

Hvis tærsklen er 3 og tre brugere flagger kommentaren, udløses agenten én gang på det tredje flag. Et fjerde, femte eller sjette flag udløser den **ikke** igen.

### Kontekst som agenten modtager

- Den flaggede kommentar.
- Valgfri tråd- / brugerhistorik- / sidekontekst som konfigureret.
- Flag-tællingen står i kommentarfeltet som `Flag Count: N`.

### Bemærk

- Triggeren udløses kun når kommentaren krydser tærsklen oppefra via platformens flag-håndteringssti (where `didIncrement === true`). Direkte DB-skrivninger, der sætter `flagCount` til tærskelværdien, udløser den ikke; flags ud over tærsklen genudløser den heller ikke.
- Den indeholder ikke, hvem der flaggede kommentaren - flags er anonyme for agenten. Hvis du vil se hvilke brugere der flaggede, hent dem fra dine egne data.
- En trigger-forsinkelse (se [Udskudte triggere](#trigger-deferred-delay)) anbefales *stærkt* for denne trigger - flags kommer ofte i klynger under en ophedet tråd, og en lille forsinkelse lader billedet lægge sig før agenten handler.

### Almindelige anvendelser

- **Moderationsgennemgang** - en flagget kommentar er det kanoniske "mennesker synes dette kan være dårligt"-signal. [Moderator template](#template-moderator) abonnerer på denne trigger som standard med en flagtærskel på 3.
- **Udvidelse af præ-moderationskø** - agenten kører et indledende tjek og markerer enten kommentaren til moderation (med `mark_comment_reviewed`) eller eskalerer den yderligere.
- **Mod brigadering** - kombiner denne trigger med [kontekst om brugerhistorik](#context-options) og lad agenten se tidligere bans/duplikatindholds-signaler før den handler.

### Anbefalinger til kombination

Abonner på **både** `COMMENT_ADD` og `COMMENT_FLAG_THRESHOLD` hvis du vil have en moderationsagent, der fanger åbenlyse tilfælde ved første øjekast og revurderer grænsetilfælde når flags akkumuleres. De to events udløses uafhængigt - agenten vil køre to gange hvis begge er abonneret og begge udløses, men den anden kørsel ser den nu-flaggede tilstand.