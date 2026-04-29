**Forfin prompt** er arbejdsprocessen til at redigere en agents [initial prompt](#personality-prompt) som svar på specifikke afgørelser, du er uenig i. Den startes fra [godkendelsesindbakken](#approval-workflow).

### Hvornår du skal bruge den

Når du gentagne gange afviser den samme type godkendelse - "agenten insisterer på at udelukke folk for brug af kraftigt sprog uden et mål" - er agentens prompt håndtaget til at rette det. Forfin prompt er en guidet måde til at:

1. Vælge en specifik godkendelse, der repræsenterer den dårlige afgørelse.
2. Redigere prompten med fuld kontekst om, hvad agenten gjorde og hvorfor.
3. Gemme den nye prompt til agenten.

Resultatet er en agent, som fremadrettet sandsynligvis ikke vil træffe den samme beslutning.

### Starte flowet

Fra godkendelsesindbakken på `/auth/my-account/ai-agent-approvals`:

1. Åbn en **afvist** godkendelse. Ruten afviser alt andet end REJECTED - pending og execution-failed godkendelser er ikke berettigede.
2. Klik på **Forfin prompt**.

Du lander på prompt-refine UI på `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Hvad siden viser

- **Godkendelsen** - agentens `toolName` og `justification` for den afviste afgørelse (den fulde LLM-transkript vises ikke her).
- **Den aktuelle prompt** - agentens gemte [initial prompt](#personality-prompt).
- **Et feedbackfelt** - du skriver **feedback** der beskriver, hvad der bør ændres (op til 2000 tegn). LLM'en genererer derefter den foreslåede nye prompt ud fra din feedback.
- **Unified inline diff** - en enkel inline diff mellem den aktuelle og den foreslåede prompt (rødt for fjernet, grønt for tilføjet).

Godkendelseskonteksten forbliver fastgjort øverst, så du kan blive ved med at henvise til "sagen jeg retter" mens du redigerer.

### Gem

Gemning opdaterer agentens `initialPrompt`-felt. Tidligere køringer (og tidligere godkendelser) bliver ikke efterkørt - den nye prompt påvirker kun fremtidige triggere. Hvis du vil verificere, at den nye prompt løser problemet, kør et [test run / replay](#test-runs-replays) mod de sidste 7 dage og se, om den nye prompt stadig ville have produceret den afviste godkendelse.

### Hvad flowet ikke gør

- Det redigerer ikke **retningslinjer for fællesskabet** - det felt har sin egen editor på hovedformularen til redigering af agenten.
- Det redigerer ikke **triggere**, **tilladte værktøjer** eller **godkendelsesgating** - disse forbliver på hovedredigeringsformularen.
- Det versionerer ikke prompten med rollback. Den tidligere prompt gemmes ikke i en separat historik-collection. Hvis du får brug for at rulle tilbage, kopér den aktuelle prompt ind i dit eget sporingssystem, før du redigerer.

### Hvorfor kombinere forfin med replay

At redigere en prompt uden at teste resultatet er baseret på tro. Den anbefalede cyklus:

1. Afvis en godkendelse.
2. Forfin prompten.
3. Kør et [test run](#test-runs-replays) mod de sidste 7 dage.
4. Se på fanen **Deltas**. Flyttede den nye prompt den dårlige beslutning væk fra "ville gøre" og ind i "ville ikke gøre"? Flyttede den ved et uheld også gode beslutninger væk?
5. Iterer.

Tre eller fire cyklusser af forfin + replay er normalt nok til at få en stabil prompt for en moderationsagent.

### Direkte redigeringsalternativ

Du behøver ikke bruge Forfin prompt - du kan også bare redigere agenten på hovedredigeringsformularen. Forfin prompt's eneste fordel er, at den fastgør en specifik fejlende sag, så du ikke mister overblikket over, hvad du retter op på.