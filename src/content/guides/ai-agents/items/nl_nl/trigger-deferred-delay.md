Standaard draait een agent **onmiddellijk** nadat zijn trigger afgaat. Het veld **Vertraging vóór uitvoering** op het bewerkingsformulier verandert dat: het platform zet de trigger in de wachtrij en voert de agent uit op het geplande tijdstip.

### When to use a delay

- **Flag-threshold triggers** - flags komen vaak in pieken binnen. Een vertraging van 10–30 minuten laat de situatie stabiliseren zodat de agent handelt op het uiteindelijke aantal flags in plaats van op het moment van binnenkomst.
- **Vote-threshold triggers** - zelfde logica, met name bij gecoördineerde neerwaartse stemacties.
- **Thread summarization** - de [Thread Summarizer template](#template-thread-summarizer) heeft standaard een vertraging van 30 minuten zodat het een gesprek samenvat dat de tijd heeft gehad zich te ontwikkelen, en niet een thread met slechts twee reacties.
- **Cool-down / re-evaluation** - "24 uur nadat een opmerking is vergrendeld, overweeg of u deze weer moet ontgrendelen."

### Configuration

- **Field**: Vertraging vóór uitvoering.
- **Range**: 0 to 2,592,000 seconds (30 days).
- **Units**: Seconds, minutes, hours, or days.

### Idempotence

De uitgestelde wachtrij dedupeert triggers niet. Twee flags die 1 seconde uit elkaar binnenkomen bij een agent met 30 minuten vertraging zullen allebei een uitvoering inplannen 30 minuten later, en de agent zal **tweemaal** draaien, beide keren op (grotendeels) dezelfde context. Als je maximaal-één-uitvoering-per-venster semantiek wilt, moet de agent dat zelf afdwingen - doorgaans door bij de eerste uitvoering een [memory note](#tools-overview) te schrijven en bij volgende uitvoeringen te controleren of die aanwezig is.

### Cost note

Uitgestelde triggers worden **opgenomen** voordat ze uitgevoerd worden. Een piek van triggers op een agent met een hoge vertraging kan zich in de wachtrij ophopen zonder tokens te verbruiken; de kosten worden pas gemaakt wanneer de cron ze afhandelt. Gebruik [Run History](#run-history) en [Drop Reasons](#drop-reasons) om te zien hoe vaak uitgestelde triggers daadwerkelijk uitgevoerd worden versus ter uitvoeringstijd worden verwijderd om budgettaire redenen.

### Replay does not honor delay

De functie [Test Runs (Replays)](#test-runs-replays) voert de agent onmiddellijk uit tegen historische opmerkingen - er wordt niet gewacht op de geconfigureerde vertraging. Beschouw dat als een functie: replays gaan over het vooraf bekijken wat de agent **zou doen** gegeven de context, niet over het reproduceren van real-time planning.