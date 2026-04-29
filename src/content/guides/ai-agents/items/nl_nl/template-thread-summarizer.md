**Template ID:** `thread_summarizer`

De Thread Summarizer plaatst een neutrale, eparagraph-samenvatting aan het einde van een lange thread. Hij gebruikt een uitstel van 30 minuten zodat de thread kan bedaren voordat de agent ernaar kijkt.

De ingebouwde prompt geeft de agent de opdracht niet te redactionaliseren - dit is cruciaal. Zonder die instructie neigt het model naar formuleringen als "naar mijn mening" die slecht lezen onder de weergavenaam van uw account.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). Zie [Deferred Triggers](#trigger-deferred-delay).

De vertraging van 30 minuten betekent dat de agent eenmaal draait, een half uur nadat een reactie is geplaatst, tegen de staat van de thread op dat moment. Het is niet "samenvatten bij elke reactie" - de wachtrij voor deferred triggers coalesceert meerdere new-comment gebeurtenissen op dezelfde thread, maar de-duplicateert ze niet over aparte vensters. U zult waarschijnlijk willen **een aangepaste regel in uw prompt toevoegen** zoals "plaats geen nieuwe samenvatting als de agent deze thread al in de afgelopen 24 uur heeft samengevat" en vertrouwen op context plus de [memory tools](#tools-overview) van de agent om dit af te dwingen.

### Allowed tools

- [`write_comment`](#tools-overview) - plaatst de samenvatting zelf.
- [`pin_comment`](#tools-overview) - pin de samenvatting zodat lezers deze bovenaan de thread zien.
- [`unpin_comment`](#tools-overview) - depint een eerdere samenvatting door dezelfde agent voordat de nieuwe gepind wordt.

De summarizer kan niet modereren of met gebruikers interageren.

### Pinning the summary

De agent plaatst een nieuwe reactie met `write_comment`, en roept vervolgens `pin_comment` aan met de teruggegeven comment ID. Bij volgende runs op dezelfde thread instrueert de prompt de agent eerst `unpin_comment` aan te roepen op zijn eerdere samenvatting - het platform zelf handhaaft **niet** een regel van één gepinde reactie per thread, dus het laten gepind blijven van de vorige samenvatting resulteert in twee gepinde samenvattingen naast elkaar. Vink "Include parent comment and prior replies in the same thread" aan in [Context Options](#context-options) zodat de agent de eerdere gepinde samenvatting kan zien.

### Recommended additions before going live

- **Vink "Include parent comment and prior replies in the same thread" aan** in [Context Options](#context-options). Een summarizer zonder thread-context is nutteloos.
- **Stem de minimum-thread-size regel af.** "Fewer than 5 comments" is de standaard in de prompt, maar in drukke communities is 10-20 geschikter. Bewerk de prompt direct.
- **Beperk tot specifieke URL-patronen** als u alleen samenvattingen op long-form pagina's wilt, niet op aankondigingen of productpagina's. Zie [Scope: URL and Locale Filters](#scope-url-locale).
- **Let op kosten.** Samenvatten is de meest token-intensieve template omdat hij de hele thread bij elke run leest. Stel een strak [daily budget](#budgets-overview) in voordat u op Enabled zet.

### Avoiding repeat summaries

De agent heeft toegang tot [`save_memory`](#tools-overview) en [`search_memory`](#tools-overview) - u kunt de prompt uitbreiden zodat hij instructies krijgt om aantekeningen zoals "summarized {thread urlId}" op te slaan en deze te controleren voordat hij opnieuw plaatst. Memory wordt gedeeld tussen alle agents in uw tenant.

---