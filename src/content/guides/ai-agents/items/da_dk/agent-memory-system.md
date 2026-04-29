Agent memory er en tenant-afgrænset, **delt** nøgle-værdi-pulje, som alle agenter i din tenant kan læse fra og skrive til. Den findes, så agenter kan bære kontekst over flere kørsler.

### Why memory exists

LLM-kontekst er per kørsel. Uden hukommelse har en agent, der giver en bruger en advarsel, ingen måde at kende til den advarsel næste gang den ser samme bruger. Plattformens eskaleringspolitik - "giv advarsel før udelukkelse" - afhænger af, at agenten kan finde den tidligere advarsel. Hukommelse er det, der får det til at fungere.

### Two kinds of memory

- **WARNING** - skrevet automatisk som en del af [`warn_user`](#tool-warn-user)-flowet. Agenten skriver ikke `WARNING`-poster manuelt; de er en bivirkning af at advare en bruger.
- **NOTE** - skrevet af [`save_memory`](#tools-overview). Generel kontekst, som agenten ønsker, at fremtidige agenter skal kende til.

Eskalationspolitikken leder specifikt efter `WARNING`-poster, når den beslutter, om en udelukkelse er berettiget.

### Tenant-scoped, agent-shared

Alle agenter i din tenant deler **én hukommelsespulje**. En note gemt af Agent A er synlig for Agent B's `search_memory`-kald. Dette er tilsigtet - du ønsker, at en triage-agents noter skal informere en moderator-agents beslutninger.

`tenantId` sættes af executor fra agentens egen tenant - aldrig fra LLM-argumenter - så hukommelseslækager på tværs af tenants er umulige efter konstruktion.

### What's in a memory record

Hver hukommelsespost indeholder:

- **Hvilken agent skrev den**, og hvornår.
- **Hvem den handler om** - den bruger, denne hukommelse beskriver. Agenten kan ikke fabrikere dette; platformen udfylder det automatisk ud fra det, der udløste agenten.
- **Et skjult alt-konto-signal** - platformen registrerer også (privat) IP-fingeraftrykket af den oprindelige kommentar, så fremtidige hukommelsessøgninger kan fremhæve noter om andre konti, der poster fra samme IP. Fingeraftrykket vises aldrig for agenten eller LLM'en.
- **Selve noten** - op til 2000 tegn frit tekst.
- **Tags** til hentning - op til 10 korte tags.
- **En slags** - enten en advarsel eller en generel note.
- **Et valgfrit kommentar-link** - hvis hukommelsen er bundet til en specifik kommentar.

### Search behavior

[`search_memory`](#tools-overview) returnerer op til 25 poster, sorteret nyest-først, og scoppes automatisk til (udløserens bruger) ELLER (andre konti på udløserens IP). Resultaterne er også begrænset til i alt 8000 tegn på tværs af alt returneret indhold - ældre indlæg droppes, hvis loftet nås.

Agenten sender ikke `userId` eller `targetIpHash`. Begge sættes af executor.

### Persistence

Hukommelse har **ingen TTL**. Poster bevares indtil de eksplicit fjernes. WARNING-poster om en bruger slettes med vilje aldrig automatisk - eskalationshistorikken skal kunne findes på ubestemt tid, ellers er platformens "søg før udelukkelse"-kontrol meningsløs.

De tre måder, hukommelse fjernes på:

- En moderator sletter den underliggende kommentar - enhver hukommelse knyttet til den kommentar kaskaderes.
- En bruger slettes - alle hukommelsesposter om den bruger fjernes i samme transaktion.
- Din tenant slettes.

Der er i dag ingen admin-brugergrænseflade til at slette individuelle hukommelsesposter.

### Memory in dry-run

Dry-run-agenter skriver **ikke** til hukommelsen. Dette er designet således: en dry-run-agents hypotetiske beslutninger bør ikke forurene den delte hukommelsespulje. Tilbage-læsning via `search_memory` virker normalt i dry-run - agenten kan se rigtige hukommelser fra live-agenter - den kan blot ikke tilføje til dem.

### Memory in replays

Samme som dry-run: replay-agenter skriver ikke hukommelse. Genafspilninger er kun til forhåndsvisning. Se [Testkørsler (Genafspilninger)](#test-runs-replays).

### Constraints summary

| Begrænsning | Værdi |
|---|---|
| Maks længde af hukommelsesindhold | 2000 tegn |
| Maks længde af hukommelses-tag | 64 tegn |
| Maks antal hukommelses-tags | 10 |
| Maks længde af hukommelsesspørgsmål | 200 tegn |
| Grænse for hukommelsessøgeresultater | 25 poster |
| Total tegnbegrænsning for hukommelsessøgningsindhold | 8000 tegn |

### See also

- [Værktøj: save_memory](#tools-overview) til skrivning.
- [Værktøj: search_memory](#tools-overview) til læsning.
- [Værktøj: warn_user](#tool-warn-user) - det eneste værktøj, der skriver hukommelse af typen WARNING.
- [Værktøj: ban_user](#tool-ban-user) - systemprompten kræver, at `search_memory` kaldes før dette.