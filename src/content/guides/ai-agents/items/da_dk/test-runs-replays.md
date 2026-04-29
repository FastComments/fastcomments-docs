---
En **testkørsel** (også kaldet en **replay**) kører agenten mod et vindue af historiske kommentarer **uden at foretage reelle handlinger**. Det er den hurtigste måde at forhåndsvise agentens adfærd, før den går live.

Tilgængelig fra agentoversigtssiden via knappen **Test run** på hver agents række.

### Hvad den gør

Platformen:

1. Vælger et udsnit af historiske kommentarer, der matcher agentens omfang, i det vindue du vælger.
2. For hver kommentar kører agenten end-to-end, som om kommentaren lige var blevet postet - samme kontekst, samme LLM-opkald, samme værktøjsvalg, samme begrundelser og samme konfidensscore.
3. Optager hver kørsel som en dry-run, tagget så den forbliver grupperet med den replay, den kom fra, og udelukkes fra live-kørende visninger.
4. **Sammenligner** agentens afgørelse med, hvad der rent faktisk skete med kommentaren - blev den senere godkendt, markeret som spam, slettet, blokeret af spam-motoren osv.

Resultatet er en diff per kommentar: "Replay-agenten ville markere dette som spam, men kommentaren er i øjeblikket godkendt og ren."

### Konfiguration

Test-run-siden har et enkelt input:

- **Dage med historiske kommentarer, der skal evalueres** - et numerisk `days` felt mellem 1 og 90. Ældre kommentarer er ikke berettigede.

Størrelsen på udvalget og den faste grænse er **ikke synlige i UI'et** - begge er server-side standarder anvendt per plan. Siden viser informationsfelter:

- **Matchende kommentarer i vinduet** - hvor mange kommentarer der ville blive betragtet.
- **Op til N kommentarer fra dette vindue vil blive behandlet** - den effektive prøvestørrelse givet server-side grænsen.
- **Anslået omkostning** - i din tenants valuta.

### Rate limit

Hver bruger er begrænset til **10 testkørsler pr. 24 timer** (rate-begrænset via nøgle `replay-create:${requestedBy}`). Knappen viser et tooltip, når du har nået grænsen ("Du har nået 10 testkørsler inden for de sidste 24 timer.").

### Concurrent kørsler

Kun én replay kan være aktiv per agent ad gangen. Hvis du starter en anden replay, mens en er i gang, omdirigeres du til den igangværende.

### Læse resultater

Når replayen er færdig, viser resultat­siden faner:

- **Deltas** (standard-aktiv) - replay-agentens afgørelse adskiller sig fra virkeligheden. (Mest interessant - "agenten ville have markeret denne kommentar som spam, men kommentaren blev godkendt og er i orden".)
- **Matches** - replay-agentens afgørelse stemmer overens med, hvad der rent faktisk skete. (Beroligende - agenten er enig med virkeligheden.)
- **No action** - replay-agenten valgte ikke at gøre noget. (Nogle gange det rigtige svar; nogle gange overså agenten noget.)
- **All** - alle resultater uanset klassifikation.

For hver kommentar i en hvilken som helst fane:

- **Tidligere udfald** - klassificeringen af, hvad der rent faktisk skete: **POSITIV**, **NEGATIV** eller **UAFKLARET**, med **Bevis** ("Kommentar markeret som slettet den {date}", "Engine: bayes", og så videre).
- **Replay agent ville** - den handling agenten valgte.
- **Hvorfor** - begrundelsen.
- **Confidence** - vist som en procentdel.

### Hvorfor replays tvinger dry-run

En replay mod en kommentar, der blev slettet for fire måneder siden, bør ikke retroaktivt slette den - den er allerede slettet. En replay mod en kommentar, som agenten nu ønsker at godkende, bør ikke ændre kommentarens nuværende tilstand. Replay er et forhåndsvisningsværktøj. At tvinge dry-run er det, der gør det sikkert at køre en replay mod ethvert historisk vindue.

### Reproducerbarhed

Replayen fryser agentens konfiguration på det tidspunkt, hvor replayen blev startet. Efterfølgende redigeringer af agenten ændrer ikke replayens resultater - resultat­siden forbliver stabil som en registrering af, hvad *den* version af agenten ville have gjort.

### Når budgetter stopper en replay

Replay er underlagt:

- Deres eget **hard cap** (sat på replay-formularen).
- Agentens daglige og månedlige **budget caps**.
- Tenantens daglige og månedlige **budget caps**.

Den første, der rammes, afbryder replayen med en specifik fejlkode. Eventuelle per-kommentar resultater, der er produceret før afbrydelsen, bevares i [Run History](#run-history).

### Hvordan replays kører

Replay kører i baggrunden, ikke synkront. Efter du klikker "Start test run", bliver replayen sat i kø, og en worker tager den op. En lang replay kan vare flere minutter. Resultatsiden poller og viser fremdrift (behandlede antal, forbruget indtil videre) løbende.

Hvis en worker dør midt i en replay, genkøer platformen automatisk replayen, så den genoptages ved næste gennemkørsel. En kort forstyrrelse efterlader aldrig en replay forældreløs.

### Hvad replay ikke gør

- **Respekterer ikke [trigger delays](#trigger-deferred-delay).** Replay kører straks, ikke 30 minutter senere.
- **Skriver ikke til hukommelsen.** Replay-agenter gemmer ikke hukommelsesnotater, selvom deres logik normalt ville gøre det.
- **Udløser ikke webhooks.** Triggere produceret af replay genererer ikke webhook-events som `trigger.succeeded` / `trigger.failed`.
- **Udelukker ikke allerede-replayede kommentarer.** At køre en anden replay mod det samme vindue dækker de samme kommentarer.

### Se også

- [Refining Prompts](#refining-prompts) - den iterative redigeringsarbejdsgang, der passer godt sammen med replays.
- [Dry-Run Mode](#dry-run-mode) - samme idé, mod live-trafik.

---