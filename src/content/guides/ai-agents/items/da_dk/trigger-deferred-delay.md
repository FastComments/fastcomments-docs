Som standard kører en agent **med det samme**, efter at dens trigger udløses. Feltet **Delay before running** på redigeringsformularen ændrer dette: platformen sætter triggeren i kø og kører agenten på det planlagte tidspunkt.

### Hvornår skal man bruge en forsinkelse

- **Flag-tærskel-triggere** - flags ankommer ofte i burst. En forsinkelse på 10-30 minutter lader billedet falde til ro, så agenten handler på det endelige antal flags frem for øjeblikket af ankomsten.
- **Vote-tærskel-triggere** - samme logik, især ved downvote-brigading.
- **Tråd-sammenfatning** - [Thread Summarizer-skabelonen](#template-thread-summarizer) bruger som standard en forsinkelse på 30 minutter, så den sammenfatter en samtale, der har haft tid til at udvikle sig, ikke en tråd med kun to svar.
- **Cool-down / revurdering** - "24 timer efter, at en kommentar er låst, overvej, om den skal låses op."

### Konfiguration

- **Felt**: Delay before running.
- **Interval**: 0 til 2.592.000 sekunder (30 dage).
- **Enheder**: Sekunder, minutter, timer eller dage.

### Idempotens

Den udskudte kø de-duplikerer ikke triggere. To flags, der ankommer 1 sekund fra hinanden på en agent med 30 minutters forsinkelse, vil begge planlægge en kørsel 30 minutter senere, og agenten vil køre **to gange**, begge gange mod (stort set) den samme kontekst. Hvis du ønsker semantik med højst én kørsel pr. vindue, skal agenten selv håndhæve det - typisk ved at skrive en [memory note](#tools-overview) ved første kørsel og tjekke for den ved efterfølgende kørsler.

### Bemærkning om omkostninger

Udskudte triggere registreres **før** de kører. En burst af triggere på en agent med lang forsinkelse kan ophobe sig i køen uden at bruge tokens; omkostningen betales først, når cron afsender dem. Brug [Run History](#run-history) og [Drop Reasons](#drop-reasons) til at se, hvor ofte udskudte triggere faktisk udføres i forhold til at blive droppet ved kørselstid af budgetmæssige årsager.

### Replay overholder ikke forsinkelsen

Funktionen [Test Runs (Replays)](#test-runs-replays) kører agenten med det samme på historiske kommentarer - den venter ikke på den konfigurerede forsinkelse. Betragt det som en feature: replays handler om at forhåndsvise, hvad agenten **ville gøre** givet konteksten, ikke om at genskabe realtids-planlægning.