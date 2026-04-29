**Skabelon-ID:** `tos_enforcer`

Moderator-skabelonen er det anbefalede udgangspunkt, hvis dit mål er at reducere manuelt moderationsarbejde. Den gennemgår nye og markerede kommentarer og anvender dine fællesskabsregler.

### Indbygget indledende prompt

[inline-code-attrs-start title = 'Moderator-skabelons indledende prompt'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Du vil næsten altid ønske at **udbygge denne prompt** med konkrete eksempler på, hvad dit site tillader og ikke tillader. Platformens egen eskalationspolitik (advar før udelukkelse, søg hukommelse før udelukkelse) er allerede indbygget i systemprompten, som agenten modtager, så du behøver ikke gentage den.

### Udløsere

- **Ny kommentar oprettet** (`COMMENT_ADD`) - agenten gennemgår hver ny kommentar.
- **Kommentar krydser en flaggrænse** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - agenten reevaluerer en kommentar, som andre brugere har markeret.

### Tilladte værktøjer

- [`mark_comment_approved`](#tools-overview) - nyttig for præ-moderationslejere, hvor agenten frigiver rene kommentarer og skjuler resten.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Den kan ikke poste kommentarer, stemme, fastgøre, låse, tildele badges eller sende e-mail - prompten er bevidst snæver.

### Anbefalede tilføjelser før udrulning

- **Angiv [Community Guidelines](#community-guidelines).** Et par sætninger med skriftlig politik er nok; agenten anvender dem ved hver kørsel.
- **Sæt `ban_user` bag [godkendelse](#approval-workflow).** Dette er aktiveret som standard i EU-regionen (se [EU DSA Article 17 Compliance](#eu-dsa-compliance)) og anbefales overalt.
- **Overvej også at sætte `mark_comment_spam` bag godkendelse** hvis du har lav volumen men højrisikoindhold.
- **Sæt `mark_comment_approved` bag godkendelse, hvis du bruger præ-moderation.** At godkende en dårlig kommentar placerer den foran læsere; sæt den bag godkendelse indtil agenten har opbygget tillid gennem dry-run.
- **Sæt kryds i "Inkluder kommentatorens tillidsfaktor, kontoalder, udelukkelseshistorik og nylige kommentarer"** i [Context Options](#context-options). Modellen vil advare langt mindre aggressivt, når den kan se, at nogen er en mangeårig godtroende bruger.

### Anbefalet dry-run-periode

Kør denne skabelon i [dry-run](#dry-run-mode) i mindst en uge mod din rigtige trafik, før du skifter til Aktiveret. Brug [Test Runs (Replays)](#test-runs-replays) til også at forhåndsvise mod de foregående 30 dage.