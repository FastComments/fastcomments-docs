**Template ID:** `welcome_greeter`

Welcome Greeter svarer varmt til brugere, der kommenterer for første gang. Det er den laveste risikoskabelon (ingen destruktive værktøjer) og en god første agent at sætte i drift.

### Indbygget startprompt

[inline-code-attrs-start title = 'Startprompt for Welcome Greeter-skabelon'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Udløsere

- **Ny bruger poster deres første kommentar på dette site** (`NEW_USER_FIRST_COMMENT`).

Denne begivenhed udløses præcis én gang per bruger, så agenten ikke kan køre i en løkke. Se [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Tilladte værktøjer

- [`write_comment`](#tools-overview)

Det er det eneste værktøj - agenten kan bogstaveligt talt ikke moderere, stemme, udelukke eller sende private beskeder.

### Anbefalede tilføjelser før du går live

- **Indstil visningsnavnet** til noget indbydende - "Community Bot", dit sites maskot, eller dit brandnavn. Visningsnavnet er det, læsere ser sammen med velkomstsvaret.
- **Sæt flueben ved "Inkluder sidetitel, undertitel, beskrivelse og meta-tags"** i [Context Options](#context-options). Greeterens svar bliver mærkbart bedre, når den kan referere til, hvad siden faktisk handler om.
- **Overvej lokalitetsbegrænsninger** hvis du opererer på flere sprog. Et velkomstsvar på det forkerte sprog er mere forstyrrende end et manglende svar. Se [Scope: URL and Locale Filters](#scope-url-locale).

### Hvorfor der ikke er behov for godkendelser

Agenten skriver kun nye kommentarer og kun ved en éngangs-udløsning. I værste fald: en akavet hilsen. Der er ingen destruktiv handling at kontrollere. De fleste operatører kører denne uden godkendelser, så snart testkørslen ser ren ud.