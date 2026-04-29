**Sjabloon-ID:** `thread_summarizer`

De Thread Samenvatter plaatst een neutrale, één-alinea samenvatting aan het einde van een lange thread. Hij gebruikt een uitstel van 30 minuten zodat de thread kan bedaren voordat de agent ernaar kijkt.

### Ingebouwde initiële prompt

[inline-code-attrs-start title = 'Aanvangprompt Thread Samenvatter-sjabloon'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

De instructie "do not editorialize" is essentieel. Zonder deze instructie neigt het model naar formuleringen als "in my view" die slecht overkomen bij de weergavenaam van uw account.

### Triggers

- **Nieuwe reactie geplaatst** (`COMMENT_ADD`).
- **Trigger-vertraging**: 30 minuten (1800 seconden). Zie [Uitgestelde triggers](#trigger-deferred-delay).

De vertraging van 30 minuten betekent dat de agent één keer draait, een half uur nadat er een reactie binnenkomt, en reageert op hoe de thread er op dat moment uitziet. Het is niet "samenvatten bij elk antwoord" — de wachtrij voor uitgestelde triggers coalesceert meerdere nieuw-reactie gebeurtenissen op dezelfde thread, maar de-dupeert ze niet over afzonderlijke vensters heen. U wilt waarschijnlijk **een aangepaste regel in uw prompt toevoegen** zoals "do not post a new summary if the agent has already summarized this thread in the last 24 hours" en vertrouwen op de context plus de agent's [geheugentools](#tools-overview) om dit af te dwingen.

### Toegestane tools

- [`write_comment`](#tools-overview) - plaatst de samenvatting zelf.
- [`pin_comment`](#tools-overview) - zet de samenvatting vast zodat lezers deze bovenaan de thread zien.
- [`unpin_comment`](#tools-overview) - verwijdert de pin van een eerdere samenvatting van dezelfde agent voordat de nieuwe wordt gepind.

De samenvatter kan niet modereren of met gebruikers interageren.

### De samenvatting pinnen

De agent plaatst een nieuwe reactie met `write_comment`, en roept vervolgens `pin_comment` aan met de geretourneerde comment-ID. Bij volgende runs op dezelfde thread instrueert de prompt de agent om eerst `unpin_comment` op zijn eerdere samenvatting aan te roepen — het platform zelf handhaaft **niet** de regel van één gepinde reactie per thread, dus het laten staan van de vorige gepinde samenvatting resulteert in twee gepinde samenvattingen naast elkaar. Vink "Include parent comment and prior replies in the same thread" aan in [Context Options](#context-options) zodat de agent de eerder gepinde samenvatting kan zien.

### Aanbevolen toevoegingen voordat u live gaat

- **Vink "Include parent comment and prior replies in the same thread" aan** in [Context Options](#context-options). Een samenvatter zonder threadcontext is nutteloos.
- **Pas de regel voor minimale threadgrootte aan.** "Minder dan 5 reacties" is de standaard in de prompt, maar in drukke communities is 10–20 meer geschikt. Bewerk de prompt rechtstreeks.
- **Beperk tot specifieke URL-patronen** als u alleen samenvattingen op longform-pagina's wilt, en niet op aankondigingen of productpagina's. Zie [Scope: URL and Locale Filters](#scope-url-locale).
- **Let op kosten.** Samenvatten is het meest token-intensieve sjabloon omdat het de hele thread bij elke run leest. Stel een strak [dagelijks budget](#budgets-overview) in voordat u op Enabled zet.

### Herhaalde samenvattingen vermijden

De agent heeft toegang tot [`save_memory`](#tools-overview) en [`search_memory`](#tools-overview) - u kunt de prompt uitbreiden zodat hij instructies krijgt om notities zoals "summarized {thread urlId}" vast te leggen en deze te controleren voordat hij opnieuw plaatst. Geheugen wordt gedeeld tussen alle agenten in uw tenant.