**Sjabloon-ID:** `top_comment_pinner`

De Top Comment Pinner houdt top-level opmerkingen in de gaten die een stemdrempel overschrijden en zet ze vast — waarbij eventuele eerder vastgezette opmerkingen in dezelfde thread worden vervangen.

### Ingebouwde initiële prompt

[inline-code-attrs-start title = 'Top Comment Pinner-sjabloon initiële prompt'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

De instructie "do not pin replies" is belangrijk: pinnen werkt op threads, dus het pinnen van een reply is zelden nuttig. De filter "non-promotional" voorkomt dat de agent een populaire link-spamopmerking extra promoot.

### Triggers

- **Een opmerking overschrijdt een stemdrempel** (`COMMENT_VOTE_THRESHOLD`, standaard stemdrempel: 10).

De trigger wordt geactiveerd wanneer de netto stemmen van de opmerking (`up - down`) de geconfigureerde drempel bereiken. Pas het getal aan op het bewerkingsformulier op basis van hoe actief uw threads zijn — 10 is een redelijke standaard voor matig actieve sites.

### Toegestane tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pinnen is niet-destructief — het kan onmiddellijk ongedaan worden gemaakt — dus dit sjabloon wordt meestal uitgevoerd zonder goedkeuringen.

### Aanbevolen aanvullingen voordat u live gaat

- **Vink "Include parent comment and prior replies in the same thread" aan** in [Context Options](#context-options). Zonder threadcontext kan de agent niet betrouwbaar bepalen of er al een vastgezette opmerking is om te ontpinnen.
- **Pas de stemdrempel aan** voor uw site. Bij drukke threads gebeurt 10 te vaak; bij rustige threads gebeurt 10 mogelijk nooit.
- **Overweeg afbakening per URL** als u alleen gepinde opmerkingen op bepaalde secties van uw site wilt — bijvoorbeeld nieuwsdiscussies, maar niet aankondigingsdiscussies.

### Opmerking over dubbele pinning

De prompt van de agent geeft opdracht eerst te ontpinnen voordat er wordt vastgezet, maar als het model die stap mist, handhaaft het platform zelf geen regel van één vastgezette opmerking per thread (u kunt er meerdere hebben). Als dubbele pinning een probleem is op uw site, plaats `pin_comment` achter een goedkeuringsstap en controleer elk geval — of schrijf een striktere prompt.

---