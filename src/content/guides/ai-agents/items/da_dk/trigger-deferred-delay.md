Som standard kører en agent **med det samme** efter at dens trigger udløses. Feltet **Delay before running** på redigeringsformularen ændrer det: platformen sætter triggeren i kø og kører agenten på det planlagte tidspunkt.

### When to use a delay

- **Flag-threshold triggers** - flags ankommer ofte i bursts. En forsinkelse på 10–30 minutter giver billedet tid til at falde til ro, så agenten handler på det endelige antal flags i stedet for øjeblikket for ankomsten.
- **Vote-threshold triggers** - samme logik, især ved downvote brigading.
- **Thread summarization** - the [Thread Summarizer template](#template-thread-summarizer) har som standard en forsinkelse på 30 minutter, så den opsummerer en samtale, der har haft tid til at udvikle sig, ikke en tråd med to svar.
- **Cool-down / re-evaluation** - "24 hours after a comment is locked, consider whether to unlock it."

### Configuration

- **Field**: Delay before running.
- **Range**: 0 til 2,592,000 seconds (30 days).
- **Units**: Seconds, minutes, hours, or days.

### Idempotence

Den udsatte kø de-duplikerer ikke triggers. To flags, der ankommer 1 sekund fra hinanden til en agent med 30 minutters forsinkelse, vil begge planlægge et kørsel 30 minutter senere, og agenten vil køre **to gange**, begge gange imod (for det meste) den samme kontekst. Hvis du vil have semantik om højst én kørsel per vindue, må agenten selv håndhæve det — typisk ved at skrive en [memory note](#tools-overview) ved første kørsel og kontrollere for den ved efterfølgende kørsel.

### Cost note

Udsatte triggers registreres **før** de kører. Et udbrud af triggers på en agent med høj forsinkelse kan hobe sig op i køen uden at bruge tokens; omkostningen betales først, når cron'en dispatcherer dem. Brug [Run History](#run-history) og [Drop Reasons](#drop-reasons) for at se, hvor ofte udsatte triggers rent faktisk eksekveres vs. bliver droppet ved kørselstid af budgetmæssige årsager.

### Replay does not honor delay

Funktionen [Test Runs (Replays)](#test-runs-replays) kører agenten med det samme mod historiske kommentarer — den venter ikke på den konfigurerede forsinkelse. Betragt det som en funktion: replays handler om at forhåndsvise, hvad agenten **ville gøre** givet konteksten, ikke om at reproducere realtidsplanlægning.