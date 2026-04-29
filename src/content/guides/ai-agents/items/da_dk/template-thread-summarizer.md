**Template ID:** `thread_summarizer`

Trådopsummereren poster et neutralt, ét-afsnits resumé i slutningen af en lang tråd. Den bruger en 30-minutters udsættelse, så tråden kan falde til ro, før agenten kigger på den.

### Indbygget startprompt

[inline-code-attrs-start title = 'Startprompt for Trådsummerer-skabelon'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

Instruktionen "do not editorialize" er afgørende. Uden den har modellen en tendens til at formulere sig som "in my view", hvilket fremstår dårligt under dit kontos visningsnavn.

### Udløsere

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). Se [Forsinkede udløsere](#trigger-deferred-delay).

30-minutters forsinkelse betyder, at agenten kører én gang, en halv time efter en kommentar lander, imod hvordan tråden ser ud på det tidspunkt. Det er ikke "opsummer ved hvert svar" - den udsatte-udløser-kø koalescerer flere nye-kommentar-hændelser på samme tråd, men de-duplicerer dem ikke på tværs af separate vinduer. Du vil sandsynligvis ønske at **tilføje en brugerdefineret regel i din prompt** som "do not post a new summary if the agent has already summarized this thread in the last 24 hours" og stole på kontekst plus agentens [memory tools](#tools-overview) til at håndhæve den.

### Tilladte værktøjer

- [`write_comment`](#tools-overview) - poster selve resuméet.
- [`pin_comment`](#tools-overview) - fæstner resuméet, så læsere ser det øverst i tråden.
- [`unpin_comment`](#tools-overview) - fjerner fæstningen af et tidligere resumé af samme agent, før den fæstner det nye.

Opsummereren kan ikke moderere eller interagere med brugere.

### Fastgørelse af resuméet

Agenten poster en ny kommentar med `write_comment`, og kalder derefter `pin_comment` med det returnerede kommentar-id. Ved efterfølgende kørsler mod samme tråd instruerer prompten den til først at kalde `unpin_comment` på sit tidligere resumé - platformen håndhæver ikke en regel om kun én fæstnet kommentar per tråd, så hvis du lader det tidligere resumé være fæstnet, vil det resultere i to fæstnede resuméer side om side. Marker "Include parent comment and prior replies in the same thread" i [Context Options](#context-options), så agenten kan se det tidligere fæstnede resumé.

### Anbefalede tilføjelser før du går live

- **Marker "Include parent comment and prior replies in the same thread"** i [Context Options](#context-options). En opsummerer uden tråd-kontekst er ubrugelig.
- **Finjuster reglen for minimum trådstørrelse.** "Færre end 5 kommentarer" er promptens standard, men i travle fællesskaber er 10–20 mere passende. Redigér prompten direkte.
- **Begræns til specifikke URL-mønstre**, hvis du kun ønsker opsummeringer på langformede sider, ikke meddelelser eller produktsider. Se [Scope: URL and Locale Filters](#scope-url-locale).
- **Hold øje med omkostningerne.** Opsummering bruger flest tokens, fordi den læser hele tråden ved hver kørsel. Sæt et stramt [dagligt budget](#budgets-overview), før du skifter til Enabled.

### Undgå gentagne resuméer

Agenten har adgang til [`save_memory`](#tools-overview) og [`search_memory`](#tools-overview) - du kan udvide prompten til at instruere den i at gemme noter som "summarized {thread urlId}" og tjekke for dem, før den poster igen. Hukommelsen deles på tværs af alle agenter i din tenant.

---