Udløser agenten, når en kommentar redigeres.

### Kontekst agenten modtager

- Kommentaren i dens aktuelle (efter-redigering) form.
- Den **forrige kommentartekst** som en separat indhegnet blok (`PREVIOUS_TEXT`). Dette er unikt for redigeringsudløseren - agenten kan sammenligne før/efter.
- Valgfri tråd-, brugerhistorik- og sidekontekst som konfigureret.

### Bemærk

- Udløseren aktiveres for enhver vellykket redigering, inklusive redigeringer udført af moderatorer på vegne af en bruger.
- Agenter har ikke noget værktøj til at redigere kommentarer tilgængeligt; agenter kan slet ikke redigere kommentarer.
- Den tidligere kommentartekst er indrammet som upålideligt input. Platformens systemprompt minder modellen om ikke at følge instruktioner fra indeni indhegningerne - det er vigtigt her, fordi en ondsindet bruger kunne redigere en kommentar for at indsætte en "ignorer dine tidligere instruktioner" payload rettet mod enhver agent, der overvåger redigeringsbegivenheder.

### Almindelige anvendelser

- **Opdage hvidvasket indhold** - en bruger redigerer en tidligere ren kommentar for at indsætte spam, efter moderatoren er gået videre.
- **Sporing af mindre redigeringer** - hvis dit fællesskab behandler redigeringer som separate hændelser til revisionsformål.

### Omkostningsnote

Redigeringsudløsere ser to kopier af kommentarens tekst (den nye version i standard COMMENT-blokken, den gamle version i PREVIOUS_TEXT-blokken). For lange kommentarer fordobler dette omtrent token-omkostningerne for kørslen sammenlignet med en `COMMENT_ADD` udløser - husk dette ved budgettering.