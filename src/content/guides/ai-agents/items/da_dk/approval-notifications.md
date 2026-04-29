Når agenten sætter en godkendelse i kø, underretter platformen anmeldere via e-mail. To indstillinger på redigeringsformularen styrer dette: **hvem** der underrettes og **hvor ofte**.

### Hvem: underretningsmode

To muligheder:

- **Alle administratorer og moderatorer** (standard) - hver kontoindehaver, superadministrator og kommentarmoderator-admin på tenant er en potentiel anmelder.
- **Specifikke brugere** - vælg en liste manuelt fra en dobbeltlistevælger på redigeringsformularen.

Under alle omstændigheder skal en kandidat til anmeldelse have en konto på tenant og en gyldig e-mailadresse for at modtage underretninger.

### Hvor ofte: frekvens pr. bruger

Hver kandidat-anmelders **egen profil** angiver deres personlige notifikationsfrekvens for agent-godkendelser:

- **Øjeblikkelig** (standard) - én e-mail pr. ventende godkendelse, sendt så snart godkendelsen oprettes.
- **Timevis** - én samlet e-mail pr. time, der opsummerer alle godkendelser sat i kø i den time.
- **Dagligt** - én samlet e-mail pr. 24 timer.
- **Deaktiveret** - ingen e-mails. Brugeren kan stadig gennemgå godkendelser via indbakke-UI; de bliver blot ikke underrettet.

Brugeren ændrer denne indstilling på sin egen profil, ikke på agentens redigeringsformular. Det er tilsigtet - én tenant kan have ti agenter, og en moderator skal ikke sætte sin foretrukne frekvens på hver agent separat.

### Cron jobs der styrer digest-mails

- **`hourly-agent-approval-digest`** - kører hver time, grupperer godkendelser sat i kø siden hver brugers sidste digest, sender én e-mail pr. bruger.
- **`daily-agent-approval-digest`** - det samme, dagligt.
- **`agent-approval-reaper`** - rydder op i godkendelser, der er ældre end 90 dage uanset tilstand.

De timelige og daglige digest-cron jobs er scoped pr. modtager: en bruger med timevis frekvens behandles af den timelige cron og springes over af den daglige (og omvendt). Brugere med øjeblikkelig frekvens underrettes af approval-create-kodevejen, ikke af crons.

### Deduplikeringsstatus

Platformen sporer, hvilke brugere der allerede er blevet mailet om hver godkendelse. Når en bruger er blevet underrettet (øjeblikkeligt eller i et digest), vil vedkommende ikke blive mailet igen om den samme godkendelse - selv hvis de ændrer deres frekvens fra øjeblikkelig til daglig midt i cyklussen.

### Godkendelse fra e-mailen

Hver underretnings-e-mail indeholder et ét-klik underskrevet login-link, der fører anmelderen direkte til godkendelsesdetaljesiden, allerede autentificeret. De kan godkende, afvise eller åbne [Forfin prompts](#refining-prompts)-flowet derfra.

### Hvad hvis der ikke findes administratorer

If `notifyMode` is `All admins and moderators` but the tenant has no super admins, comment moderator admins, or account owners with valid emails, the platform logs a warning and the approval still queues - just nobody gets notified about it. It will sit in the inbox until someone happens to look.

If `notifyMode` is `Specific users` but you have not selected any users, same outcome.

### Hvad hvis faktureringsnotifikationer er deaktiveret

[Budget Alerts](#budget-alerts) - de budgetrelaterede e-mails - går til faktureringsadministratorer **uanset brugerens individuelle notifikationspræference**. Dette er tilsigtet: budgetoverskridelser påvirker omkostningerne, og ejeren af faktureringen skal vide det.

Approval-notifikationer respekterer kun den per-bruger agent-approval-frekvensindstilling. De tjekker ikke den bredere afmelding af admin-notifikationer - en bruger, der har afmeldt admin-notifikationer, vil stadig modtage godkendelses-e-mails, hvis vedkommende er på anmelderlisten, medmindre deres agent-approval-frekvens er sat til **Deaktiveret**.

### Se også

- [Approval Workflow](#approval-workflow) for hele livscyklussen af en godkendelse.
- [Forfin prompts](#refining-prompts) for arbejdsgangen "I keep approving the same kind of mistake".

---