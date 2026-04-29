**Skabelon-ID:** `tos_enforcer`

Moderator-skabelonen er det anbefalede udgangspunkt, hvis dit mål er at reducere manuel moderation. Den gennemgår nye og markerede kommentarer og anvender dine fællesskabsregler.

Du vil næsten altid ønske at **udbygge den indbyggede prompt** med konkrete eksempler på, hvad dit site tillader og ikke tillader. Platformens egen eskaleringspolitik (advar før udelukkelse, søg i hukommelsen før udelukkelse) er allerede indbygget i systemprompten, som agenten modtager, så du behøver ikke gentage den.

### Triggers

- **Ny kommentar oprettet** (`COMMENT_ADD`) - agenten ser på hver ny kommentar.
- **Kommentar når en flagtærskel** (`COMMENT_FLAG_THRESHOLD`, standardtærskel: 3) - agenten revurderer en kommentar, som andre brugere har markeret.

### Tilladte værktøjer

- [`mark_comment_approved`](#tools-overview) - nyttigt for pre-moderation-lejere, hvor agenten frigiver rene kommentarer og skjuler resten.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Den kan ikke poste kommentarer, stemme, fastgøre, låse, tildele badges eller sende e-mail — prompten er bevidst snæver.

### Anbefalede tilføjelser før produktionssætning

- **Angiv [Community Guidelines](#community-guidelines).** Et par sætninger af skriftlig politik er nok; agenten anvender dem ved hver kørsel.
- **Sæt `ban_user` bag en [godkendelse](#approval-workflow).** Dette er aktiveret som standard i EU-regionen (se [EU DSA Artikel 17-overholdelse](#eu-dsa-compliance)) og anbefales overalt.
- **Overvej også at sætte `mark_comment_spam` bag godkendelse** hvis du har lav volumen, men højt indholdssensitivt indhold.
- **Sæt `mark_comment_approved` bag godkendelse, hvis du kører pre-moderation.** Godkendelse af en dårlig kommentar viser den for læsere; lås det, indtil agenten har vundet tillid gennem dry-run.
- **Sæt flueben ved "Include commenter's trust factor, account age, ban history, and recent comments"** i [Kontekstindstillinger](#context-options). Modellen vil advare langt mindre aggressivt, når den kan se, at nogen er en mangeårig bruger med god tro.

### Anbefalet dry-run-periode

Kør denne skabelon i [dry-run](#dry-run-mode) i mindst en uge mod din reelle trafik, før du skifter til Enabled. Brug [Testkørsler (Genspil)](#test-runs-replays) til også at forhåndsvise imod de foregående 30 dage.

---