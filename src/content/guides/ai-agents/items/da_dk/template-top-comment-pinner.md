**Skabelon-ID:** `top_comment_pinner`

Top Comment Pinner overvåger topniveau-kommentarer, der krydser en stemmetærskel, og fastgør dem - og erstatter det, der tidligere var fastgjort i samme tråd.

Den indbyggede prompt instruerer agenten til at springe svar over (fastgørelse virker på tråde, så det er sjældent nyttigt at fastgøre et svar) og til at filtrere promoverende indhold (så agenten ikke fremmer populær link-spam).

### Udløsere

- **En kommentar krydser en stemmetærskel** (`COMMENT_VOTE_THRESHOLD`, standard stemmetærskel: 10).

Udløseren aktiveres, når kommentarens netto-stemmer (`up - down`) når den konfigurerede tærskelværdi. Juster tallet i redigeringsformularen baseret på hvor aktive dine tråde er - 10 er en fornuftig standard for moderat aktive sider.

### Tilladte værktøjer

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fastgørelse er ikke-destruktiv - den kan fortrydes øjeblikkeligt - så denne skabelon kører normalt uden godkendelser.

### Anbefalede tilføjelser før du går live

- **Markér "Inkluder overordnet kommentar og tidligere svar i samme tråd"** i [Context Options](#context-options). Uden tråd-kontekst kan agenten ikke pålideligt afgøre, om der allerede er en fastgjort kommentar, der skal fjernes.
- **Juster stemmetærsklen** til dit websted. På travle tråde vil 10 ofte være for lavt; på stille tråde vil 10 måske aldrig blive nået.
- **Overvej at afgrænse efter URL** hvis du kun vil have fastgjorte kommentarer i bestemte sektioner af dit websted - for eksempel nyhedstråde, men ikke annoncetråde.

### Bemærkning om duplikeret fastgørelse

Agentens prompt instruerer den til først at afpine før den fastgør, men hvis modellen overser det trin, håndhæver platformen ikke en regel om kun én fastgjort kommentar per tråd (du kan have flere). Hvis duplikeret fastgørelse er et problem på dit websted, kræv godkendelse for `pin_comment` og gennemgå hver enkelt - eller skriv en strammere prompt.