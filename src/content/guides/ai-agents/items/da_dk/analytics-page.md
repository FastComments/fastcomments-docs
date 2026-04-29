Analyser er dashboardet på tværs af agenter. Tilgængeligt fra AI-agenter-siden via fanen **Analyser** (på tenant-niveau) eller per agent via knappen **Analyser** på hver agents række.

### Filter

En dropdown øverst - **Alle agenter** eller en specifik agent. Resten af siden tilpasser sig derefter.

### Budgetforbrug

Fire fremdriftslinjer, der viser forbruget i den aktuelle periode i forhold til grænsen:

- **Agent i dag** (når filtreret til en specifik agent) - daglig agentgrænse.
- **Agent denne måned** - månedlig agentgrænse.
- **Konto i dag** - tenantens daglige grænse.
- **Konto denne måned** - tenantens månedlige grænse.

Når en grænse ikke er sat, viser baren "(ingen grænse sat)" og viser det rå forbrug.

### Daglige omkostninger (seneste 30 dage)

En tabel med dagsomkostninger i tenantens valuta for det valgte omfang. Nyttigt til at opdage:

- **Pludselige omkostningsspidser** - normalt fra en løbsk løkke eller en viral kommentar, der spreder sig og udløser triggers.
- **Omkostningsdrift** - gradvist stigende daglige omkostninger i takt med, at dit community vokser.

### Udførte handlinger

En opdeling af handlingstyper over den aktuelle måned - "Skrev en kommentar: 47", "Markerede en kommentar som spam: 12" osv. Nyttigt til at tjekke, at agenten gør, hvad du forventede.

### Triggere sprunget over (denne måned)

Tællinger grupperet efter [årsag til drop](#drop-reasons):

- Over agentens daglige / agentens månedlige / kontoens daglige / kontoens månedlige.
- Rate-begrænset.
- Samtidighed overbelastet.

Hvis du ser drops her, rammer din agent en budget- eller rategrænse og går glip af triggere, den ellers ville have kørt på. Se [Årsager til drop](#drop-reasons).

### Tørkørsel vs live (denne måned)

- **Aktiverede kørsler** - antal kørsler, der udførte reelle handlinger denne måned.
- **Tørkørsler** - antal kørsler i dry-run-tilstand denne måned.

Et nyttigt tuning-signal: en helt ny agent, som endnu ikke er blevet sat til Aktiveret, vil kun vise tørkørsler. En agent i Aktiveret-tilstand med nul i alle tællinger i denne sektion sidder inaktiv - enten bliver den ikke udløst, den er udelukket af omfang, eller dens triggere er ikke konfigureret korrekt.

### Top-agenter efter månedlige omkostninger

Når filteret er **Alle agenter**, viser siden agenter rangeret efter månedens omkostninger til dato. At finde din dyreste agent er det første skridt i omkostningsoptimering - normalt er svaret "stram op på dets [kontekstindstillinger](#context-options)" eller "sænk dets [budgetgrænse](#budgets-overview)".

### Agenter ved eller nær deres grænse

Pr. agent-opdeling af agenter, hvis forbrug er ved eller tæt på deres per-agent grænser i den aktuelle periode:

- **tæt på grænsen** - over en konfigurerbar procentdel af grænsen.
- **over grænsen** - faktisk begrænset, med `{count} dropped` triggere i den periode.

Klik ind på agenten fra denne tabel for at hæve grænsen, indsnævre omfanget eller sætte den på pause.

### Kontosammendrag

Når filteret er **Alle agenter**:

- **Triggere i dag** - antal.
- **Triggere denne måned** - antal.
- For hver: et suffix `dropped` der viser, hvor mange der blev sprunget over.

### Valuta

Alle pengeværdier vises i tenantens valuta.

### Hvad denne side ikke gør

- Den viser ikke **omkostningsopdelinger pr. handling** - de findes i [Detaljevisning for kørsel](#run-detail-view).
- Den viser ikke **transskripter** eller **LLM-svar**.
- Den giver dig ikke mulighed for at handle på agenter - redigering, pause og sletning foretages alle fra agentlisten / redigeringssiden.