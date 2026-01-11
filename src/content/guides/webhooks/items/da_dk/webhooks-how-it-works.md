---
Alle ændringer af Comment-objektet i systemet udløser en begivenhed, som ender i en kø.

Den indledende webhook-begivenhed sendes normalt inden for seks sekunder efter, at begivenhedskilden indtræffer.

Du kan overvåge denne kø i Webhooks-administrationen i tilfælde af, at din API går ned.

Hvis en anmodning til din API fejler, vil vi sætte den i kø igen efter en tidsplan.

Den tidsplan er `1 Minute * the retry count`. Hvis kaldet fejler én gang, forsøger det igen om
et minut. Hvis det fejler to gange, vil det derefter vente to minutter, og så videre. Dette er for at undgå, at vi
overbelaster din API, hvis den går ned på grund af belastning.

Webhooks kan annulleres fra [log-siden](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).

---