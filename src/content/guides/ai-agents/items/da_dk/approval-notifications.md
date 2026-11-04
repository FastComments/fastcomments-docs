Når agenten sætter en godkendelse i kø, underretter platformen anmeldere via e-mail. To indstillinger på redigeringsformularen styrer dette: **hvem** der underrettes, og **hvor ofte**.

### Hvem: underretningstilstand

To tilstande:

- **Alle administratorer og moderatorer** (standard) - hver konto-ejer, super-administrator og kommentarmoderator-administrator på tenanten er en kandidat-anmelder.
- **Specifikke brugere** - udvælg manuelt en liste fra en dobbeltliste-vælger på redigeringsformularen.

Uanset hvad skal en kandidat-anmelder have en konto på tenanten og en gyldig e-mailadresse for at modtage underretninger.

### Hvor ofte: frekvens pr. bruger

Hver kandidat-anmelders **egen profil** angiver deres personlige underretningsfrekvens for agent-godkendelser:

- **Øjeblikkelig** (standard) - én e-mail pr. afventende godkendelse, sendt så snart godkendelsen oprettes.
- **Hver time** - én opsummerings-e-mail i timen, der opsummerer alle godkendelser, der er sat i kø i den time.
- **Dagligt** - én opsummerings-e-mail pr. 24 timer.
- **Deaktiveret** - ingen e-mails. Brugeren kan stadig gennemgå godkendelser via inbox-UI'en; de bliver bare ikke pinget.

Brugeren ændrer denne indstilling på deres egen profil, ikke på agentens redigeringsformular. Dette er bevidst - en tenant kan have ti agenter, og en moderator bør ikke skulle indstille deres foretrukne frekvens på hver agent separat.

### Cron-jobs, der driver opsummeringer

- **`hourly-agent-approval-digest`** - kører hver time, batcher godkendelser, der er sat i kø siden hver brugers sidste opsummering, og sender én e-mail pr. bruger.
- **`daily-agent-approval-digest`** - det samme, dagligt.
- **`agent-approval-reaper`** - fjerner godkendelser, der er ældre end 90 dage, uanset tilstand.

De time- og daglige opsummerings-crons er afgrænset pr. modtager: en bruger med timefrekvens behandles af time-cron'en og springes over af den daglige (og omvendt). Brugere med øjeblikkelig frekvens underrettes via kodevejen for oprettelse af godkendelser, ikke af crons'ene.

### Dedup-tilstand

Platformen registrerer, hvilke brugere der allerede er blevet mailet om hver godkendelse. Når en bruger er blevet underrettet (øjeblikkeligt eller i en opsummering), vil de ikke blive mailet igen om den samme godkendelse - selv hvis de ændrer deres frekvens fra øjeblikkelig til daglig midt i en cyklus.

### Godkendelse fra e-mailen

Hver underretnings-e-mail indeholder et one-click signeret login-link, der fører anmelderen direkte til godkendelsens detaljeside, allerede autentificeret. Derfra kan de godkende, afvise eller åbne [Refine Prompts](#refining-prompts)-flowet.

### Hvad hvis der ikke findes administratorer

Hvis `notifyMode` er `Alle administratorer og moderatorer`, men tenanten ikke har super-administratorer, kommentarmoderator-administratorer eller konto-ejere med gyldige e-mails, logger platformen en advarsel, og godkendelsen sættes stadig i kø - der bliver bare ikke underrettet nogen om den. Den vil ligge i inboxen, indtil nogen tilfældigt kigger.

Hvis `notifyMode` er `Specifikke brugere`, men du ikke har valgt nogen brugere, er resultatet det samme.

### Hvad hvis faktureringsunderretninger er deaktiveret

[Budget Alerts](#budget-alerts) - de budgetrelaterede e-mails - går til faktureringsadministratorer **uanset den pr.-bruger-underretningspræference**. Dette er bevidst: budgetoverskridelser påvirker omkostningerne, og faktureringsejeren skal vide det.

Underretninger om godkendelser følger kun indstillingen for pr.-bruger-frekvens for agent-godkendelser. De tjekker ikke den bredere opt-out for administratorunderretninger - en bruger, der har frameldt sig administratorunderretninger, vil stadig modtage godkendelses-e-mails, hvis de er på anmelderlisten, medmindre deres agent-godkendelsesfrekvens er sat til **Deaktiveret**.

### Se også

- [Approval Workflow](#approval-workflow) for den fulde livscyklus for en godkendelse.
- [Refining Prompts](#refining-prompts) for "jeg bliver ved med at godkende den samme slags fejl"-workflowet.