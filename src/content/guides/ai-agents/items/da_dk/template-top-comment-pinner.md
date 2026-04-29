**Skabelon-ID:** `top_comment_pinner`

Top Comment Pinner overvåger topniveau-kommentarer, der krydser en stemmetærskel, og fastgør dem - og erstatter dermed det, der tidligere var fastgjort i samme tråd.

### Indbygget startprompt

[inline-code-attrs-start title = 'Startprompt for Top Comment Pinner-skabelon'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

Instruktionen "do not pin replies" er vigtig: fastgøring fungerer på tråde, så det er sjældent nyttigt at fastgøre et svar. Filteret "non-promotional" forhindrer agenten i at booste en populær link-spam-kommentar.

### Udløsere

- **En kommentar krydser en stemmetærskel** (`COMMENT_VOTE_THRESHOLD`, standard stemmetærskel: 10).

Udløseren affyres, når kommentarens netto-stemmer (`up - down`) når den konfigurerede tærskel. Juster tallet i redigeringsformularen baseret på, hvor aktive dine tråde er - 10 er en fornuftig standard for moderat aktive sites.

### Tilladte værktøjer

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fastgøring er ikke-destruktiv - den kan øjeblikkeligt omstilles - så denne skabelon kører normalt uden godkendelser.

### Anbefalede tilføjelser før lancering

- **Sæt kryds ved "Inkluder forældrekommentar og tidligere svar i samme tråd"** i [Context Options](#context-options). Uden trådkontekst kan agenten ikke pålideligt afgøre, om der allerede er en fastgjort kommentar, der skal fjernes.
- **Justér stemmetærsklen** til dit site. På travle tråde sker 10 for ofte; på stille tråde sker 10 måske aldrig.
- **Overvej at afgrænse efter URL** hvis du kun vil have fastgjorte kommentarer i visse sektioner af dit site - f.eks. nyhedstråde, men ikke annoncestråde.

### Bemærkning om dobbelt fastgøring

Agentens prompt instruerer den i først at fjerne den tidligere fastgørelse, før den fastgør, men hvis modellen overser det trin, håndhæver platformen ikke selv en regel om én fastgjort per tråd (du kan have flere). Hvis dobbelt fastgøring er et problem på dit site, placer `pin_comment` bag en godkendelse og gennemse hver enkelt - eller skriv en snævrere prompt.