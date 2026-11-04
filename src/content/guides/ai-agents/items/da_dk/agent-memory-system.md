Agenthukommelse er en tenant-afgrænset, **delt** key-value-pool, som alle agenter i din tenant kan læse fra og skrive til. Den findes, så agenter kan bære kontekst med sig på tværs af kørsler.

### Hvorfor hukommelse findes

LLM-kontekst er pr. kørsel. Uden hukommelse har en agent, der udsteder en advarsel til en bruger, ingen mulighed for at vide om den advarsel, næste gang den ser samme bruger. Platformens eskaleringspolitik - "advar før udelukkelse" - afhænger af, at agenten kan finde den tidligere advarsel. Hukommelse er det, der gør det muligt.

### To typer hukommelse

- **WARNING** - skrives automatisk som en del af [`warn_user`](#tool-warn-user)-flowet. Agenten skriver ikke `WARNING`-poster manuelt; de er en bivirkning af at advare en bruger.
- **NOTE** - skrives af [`save_memory`](#tools-overview). Generel kontekst, som agenten ønsker, at fremtidige agenter skal kende.

Eskaleringspolitikken ser specifikt efter `WARNING`-poster, når den afgør, om en udelukkelse er berettiget.

### Tenant-afgrænset, delt mellem agenter

Alle agenter i din tenant deler **én hukommelsespool**. En note gemt af Agent A er synlig for Agent B's `search_memory`-kald. Dette er bevidst - du vil have, at en triage-agents noter skal informere en moderator-agents beslutninger.

`tenantId` sættes af eksekveren ud fra agentens egen tenant - aldrig fra LLM-argumenter - så lækager af hukommelse på tværs af tenants er umulige af konstruktion.

### Hvad en hukommelsespost indeholder

Hver hukommelsespost indeholder:

- **Hvilken agent der skrev den**, og hvornår.
- **Hvem den handler om** - brugeren, som denne hukommelse beskriver. Agenten kan ikke fabrikere dette; platformen udfylder det automatisk ud fra, hvad der udløste agenten.
- **Et skjult alt-account-signal** - platformen registrerer også (privat) IP-fingerprintet af den oprindelige kommentar, så fremtidige hukommelsessøgninger kan fremvise noter om andre konti, der poster fra samme IP. Fingerprintet vises aldrig til agenten eller LLM'en.
- **Selve noten** - op til 2000 tegn frit tekst.
- **Tags** til søgning - op til 10 korte tags.
- **En type** - enten en advarsel eller en generel note.
- **Et valgfrit kommentarlink** - hvis hukommelsen er knyttet til en specifik kommentar.

### Søgeadfærd

[`search_memory`](#tools-overview) returnerer op til 25 poster, sorteret nyeste først, automatisk afgrænset til (udløserens bruger) ELLER (andre konti på udløserens IP). Resultaterne er også tegnbegrænsede til i alt 8000 tegn på tværs af alt returneret indhold - ældre poster droppes, hvis grænsen nås.

Agenten angiver ikke `userId` eller `targetIpHash`. Begge sættes af eksekveren.

### Persistens

Hukommelse har **ingen TTL**. Poster består, indtil de eksplicit fjernes. WARNING-poster om en bruger slettes bevidst aldrig automatisk - eskaleringshistorikken skal kunne findes på ubestemt tid, ellers er platformens "søg før udelukkelse"-tjek meningsløst.

De tre måder, hukommelse fjernes på:

- En moderator sletter den underliggende kommentar - al hukommelse knyttet til den kommentar kaskades.
- En bruger slettes - alle hukommelsesposter om den bruger fjernes i samme transaktion.
- Din tenant slettes.

Der findes i dag ingen admin-UI til at slette individuelle hukommelsesposter.

### Hukommelse i dry-run

Dry-run-agenter skriver **ikke** hukommelse. Dette er med vilje: en dry-run-agents hypotetiske beslutninger må ikke forurene den delte hukommelsespool. Læsning via `search_memory` fungerer normalt i dry-run - agenten kan se rigtige hukommelser fra live-agenter - den kan bare ikke tilføje til dem.

### Hukommelse i replays

Samme som dry-run: replay-agenter skriver ikke hukommelse. Replays er kun forhåndsvisning. Se [Test Runs (Replays)](#test-runs-replays).

### Oversigt over begrænsninger

| Grænse | Værdi |
|---|---|
| Maks. længde af hukommelsesindhold | 2000 tegn |
| Maks. længde af hukommelsestag | 64 tegn |
| Maks. antal hukommelsestags | 10 |
| Maks. længde af hukommelsesforespørgsel | 200 tegn |
| Grænse for hukommelsessøgeresultater | 25 poster |
| Samlet indholdsgrænse for hukommelsessøgning | 8000 tegn |

### Se også

- [Værktøj: save_memory](#tools-overview) til skrivning.
- [Værktøj: search_memory](#tools-overview) til læsning.
- [Værktøj: warn_user](#tool-warn-user) - det eneste værktøj, der skriver WARNING-type hukommelse.
- [Værktøj: ban_user](#tool-ban-user) - systemprompten kræver, at `search_memory` kaldes før dette.