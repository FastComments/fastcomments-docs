Udløses, når en kommentar slettes.

### Kontekst, som agenten modtager

- Kommentaren, som netop blev slettet - tekst, forfatter, side.
- Valgfri tråd-, brugerhistorik- eller sidekontekst som konfigureret.

### Bemærk

- Udløses for både **bløde sletninger** (hvor kommentaren skjules, men beholdes til revision) og **hårde sletninger** (hvor kommentaren fjernes fuldstændigt). Trigger-handleren opløser kommentaren fra kaskadesletnings-pipelinen; hvad agenten ser, er den sidst kendte tilstand.
- Når en kommentar er fuldstændigt slettet, vil værktøjer, der retter sig mod den (`pin_comment`, `mark_comment_spam`, etc.) for det pågældende kommentar-ID fejle.

### Typiske anvendelser

- **Audit-videresendelse via [Webhooks](#webhooks-overview)** - udsend et `trigger.succeeded`-event, så et eksternt system registrerer, hvad der blev slettet.
- **Skrivninger til hukommelsen** - få agenten til at registrere et [hukommelsesnotat](#tools-overview) om et sletningsmønster (den slettede kommentar var brugerens tredje inden for 24 timer, osv.).
- **Tværtråds-effekter** - bemærk, når en sletning ændrer strukturen i en tråd, som agenten tidligere har opsummeret, og overvej, om der skal genopsummeres.

### Bemærkning om driftsomkostninger

Hvis du har et websted med høj sletningsvolumen (omfattende menneskelig moderation), kan denne trigger udløses ofte.

---