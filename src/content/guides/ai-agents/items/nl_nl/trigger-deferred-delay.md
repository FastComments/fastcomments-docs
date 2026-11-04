Standaard wordt een agent **onmiddellijk** uitgevoerd nadat zijn trigger afgaat. Het veld **Delay before running** op het bewerkingsformulier verandert dat: het platform zet de trigger in de wachtrij en voert de agent uit op het geplande tijdstip.

### Wanneer een vertraging gebruiken

- **Vlagdrempel-triggers** - vlaggen komen vaak in bursts binnen. Een vertraging van 10-30 minuten laat het beeld tot rust komen, zodat de agent handelt op basis van het uiteindelijke aantal vlaggen in plaats van op het moment van binnenkomst.
- **Stemdrempel-triggers** - dezelfde logica, met name voor downvote-brigading.
- **Thread-samenvatting** - het [Thread Summarizer template](#template-thread-summarizer) gebruikt standaard een vertraging van 30 minuten, zodat het een gesprek samenvat dat de tijd heeft gehad om zich te ontwikkelen, niet een thread met slechts twee reacties.
- **Cool-down / herbeoordeling** - "24 uur nadat een reactie is vergrendeld, overweeg of deze ontgrendeld moet worden."

### Configuratie

- **Veld**: Delay before running.
- **Bereik**: 0 tot 2.592.000 seconden (30 dagen).
- **Eenheden**: Seconden, minuten, uren of dagen.

### Idempotentie

De uitgestelde wachtrij dedupliceert geen triggers. Twee vlaggen die 1 seconde na elkaar binnenkomen op een agent met 30 minuten vertraging zullen beide een run 30 minuten later inplannen, en de agent wordt **twee keer** uitgevoerd, beide keren tegen (grotendeels) dezelfde context. Als je at-most-one-run-per-window-semantiek wilt, moet de agent dit zelf afdwingen - doorgaans door bij de eerste run een [memory note](#tools-overview) te schrijven en bij volgende runs daarnaar te controleren.

### Kostennotitie

Uitgestelde triggers worden **vóór** ze worden uitgevoerd geregistreerd. Een burst van triggers op een agent met lange vertraging kan zich opstapelen in de wachtrij zonder tokens uit te geven; de kosten worden pas betaald wanneer de cron ze verzendt. Gebruik [Run History](#run-history) en [Drop Reasons](#drop-reasons) om te zien hoe vaak uitgestelde triggers daadwerkelijk worden uitgevoerd versus worden weggegooid tijdens de run om budgetredenen.

### Replay houdt geen rekening met vertraging

De functie [Test Runs (Replays)](#test-runs-replays) voert de agent onmiddellijk uit op historische reacties - hij wacht niet op de geconfigureerde vertraging. Behaal dat als een feature: replays gaan over het vooraf bekijken wat de agent **zou doen** gegeven de context, niet over het reproduceren van realtime planning.