En **testkørsel** (også kaldet en **replay**) kører agenten mod et vindue af historiske kommentarer **uden at foretage rigtige handlinger**. Det er den hurtigste måde at forhåndsvise agentens adfærd, før den går i drift.

Tilgængelig fra agents-listesiden via knappen **Test run** på hver agents række.

### Hvad den gør

Platformen:

1. Vælger et udvalg af historiske kommentarer, der matcher agentens scope, i det vindue, du vælger.
2. For hver kommentar kører agenten end-to-end, som om kommentaren lige var blevet postet - samme kontekst, samme LLM-kald, samme værktøjsvalg, samme begrundelser og confidence-scores.
3. Registrerer hver kørsel som en dry-run, mærket så den forbliver grupperet med den replay, den kom fra, og ekskluderet fra live-run-visninger.
4. **Sammenligner** agentens dom med, hvad der faktisk skete med kommentaren - blev den senere godkendt, markeret som spam, slettet, blokeret af spam-motoren osv.

Resultatet er en pr. kommentar-diff: "Replay-agenten ville markere denne som spam, men kommentaren er i øjeblikket godkendt og ren."

### Konfiguration

Testkørsel-siden har et enkelt input:

- **Dage med historiske kommentarer at evaluere** - et numerisk `days`-felt mellem 1 og 90. Ældre kommentarer er ikke berettigede.

Størrelsen på udvalget og hård grænse er **ikke eksponeret i UI'en** - begge er server-side-standarder anvendt pr. plan. Siden viser informationsfelter:

- **Matchende kommentarer i vinduet** - hvor mange kommentarer der ville blive overvejet.
- **Op til N kommentarer fra dette vindue vil blive behandlet** - den effektive størrelse på udvalget givet den server-side grænse.
- **Estimeret omkostning** - i din tenants valuta.

### Rate limit

Hver bruger er begrænset til **10 testkørsler pr. 24 timer** (rate-limited via nøglen `replay-create:${requestedBy}`). Knappen viser et tooltip, når du har nået grænsen ("Du har nået 10 testkørsler i de sidste 24 timer.").

### Samtidighed

Kun én replay kan være aktiv pr. agent ad gangen. Hvis du starter en anden replay, mens en er i gang, omdirigeres du til den igangværende.

### Læsning af resultater

Når replay'en afsluttes, viser resultatsiden faner:

- **Deltas** (standard aktiv) - replay-agentens dom afviger fra virkeligheden. (Mest interessant - "agenten ville have markeret denne kommentar som spam, men kommentaren blev godkendt og er fin".)
- **Matches** - replay-agentens dom matcher, hvad der faktisk skete. (Beroligende - agenten er enig med virkeligheden.)
- **No action** - replay-agenten besluttede at gøre ingenting. (Nogle gange det rigtige svar; nogle gange overså agenten noget.)
- **All** - alle resultater uanset klassificering.

For hver kommentar i enhver fane:

- **Prior outcome** - klassificeringen af, hvad der faktisk skete: **POSITIVE**, **NEGATIVE** eller **INDETERMINATE**, med **Evidence** ("Comment marked deleted at {date}", "Engine: bayes" osv.).
- **Replay agent would** - den handling, agenten valgte.
- **Why** - begrundelsen.
- **Confidence** - vist som en procentdel.

### Hvorfor replays tvinger dry-run igennem

En replay mod en kommentar, der blev slettet for fire måneder siden, bør ikke retroaktivt slette den - den er allerede slettet. En replay mod en kommentar, som agenten nu vil godkende, bør ikke ændre kommentarens nuværende tilstand. Replay er et forhåndsvisningsværktøj. Det er gennemtvingelsen af dry-run, der gør det sikkert at køre en replay mod ethvert vindue af historik.

### Reproducerbarhed

Replays fryser agentens konfiguration i det øjeblik, replay'en blev startet. Efterfølgende redigeringer af agenten ændrer ikke replay'ens resultater - resultatsiden forbliver stabil som en registrering af, hvad *den* version af agenten ville have gjort.

### Når budgetter stopper en replay

Replays er underlagt:

- Deres egen **hårde grænse** (sat på replay-formularen).
- Agentens daglige og månedlige **budgetgrænser**.
- Tenantens daglige og månedlige **budgetgrænser**.

Den første, der nås, afbryder replay'en med en specifik fejlkode. Alle pr. kommentar-resultater, der er produceret før afbrydelsen, bevares i [Run History](#run-history).

### Hvordan replays kører

Replays kører i baggrunden, ikke synkront. Når du klikker på "Start test run", sættes replay'en i kø, og en worker henter den. En lang replay kan vare flere minutter. Resultatsiden poller og viser fremskridt (behandlet antal, forbrug indtil videre), mens den skrider frem.

Hvis en worker dør midt i en replay, sætter platformen automatisk replay'en i kø igen, så den genoptages i næste gennemløb. Et kort afbrydelse efterlader aldrig en replay forældreløs.

### Hvad replay ikke gør

- **Overholder ikke [trigger delays](#trigger-deferred-delay).** Replays kører straks, ikke 30 minutter senere.
- **Skriver ikke til hukommelsen.** Replay-agenter gemmer ikke memory notes, selvom deres logik normalt ville.
- **Udløser ikke webhooks.** Replays-producerede triggers genererer ikke `trigger.succeeded` / `trigger.failed` webhook-hændelser.
- **Ekskluderer ikke allerede-replayede kommentarer.** At køre en anden replay mod det samme vindue dækker de samme kommentarer.

### Se også

- [Refining Prompts](#refining-prompts) - den iterative redigerings-workflow, der passer godt sammen med replays.
- [Dry-Run Mode](#dry-run-mode) - samme idé, mod live-trafik.