**Template-ID:** `thread_summarizer`

De Thread Summarizer plaatst een neutrale samenvatting van één alinea aan het einde van een lange thread. Het gebruikt een uitstel van 30 minuten zodat de thread kan bezinken voordat de agent ernaar kijkt.

De ingebouwde prompt instrueert de agent om niet te redactioneren - dit is essentieel. Zonder dit grijpt het model naar "naar mijn mening"-formuleringen die slecht tot hun recht komen onder de weergavenaam van je account.

### Triggers

- **Nieuwe reactie geplaatst** (`COMMENT_ADD`).
- **Trigger-vertraging**: 30 minuten (1800 seconden). Zie [Uitgestelde triggers](#trigger-deferred-delay).

De vertraging van 30 minuten betekent dat de agent één keer draait, een half uur nadat een reactie is geplaatst, tegen de thread zoals die op dat moment is. Het is niet "samenvatten bij elke reactie" - de wachtrij voor uitgestelde triggers bundelt meerdere nieuwe-reactie-gebeurtenissen op dezelfde thread, maar dedupliceert ze niet over aparte vensters. Je zult waarschijnlijk **een aangepaste regel in je prompt willen toevoegen** zoals "plaats geen nieuwe samenvatting als de agent deze thread al in de afgelopen 24 uur heeft samengevat" en vertrouwen op context plus de [geheugentools](#tools-overview) van de agent om dit af te dwingen.

### Toegestane tools

- [`write_comment`](#tools-overview) - plaatst de samenvatting zelf.
- [`pin_comment`](#tools-overview) - pinnt de samenvatting zodat lezers deze bovenaan de thread zien.
- [`unpin_comment`](#tools-overview) - ont-pinnt een eerdere samenvatting van dezelfde agent voordat de nieuwe wordt gepind.

De summarizer kan niet modereren of met gebruikers communiceren.

### De samenvatting pinnen

De agent plaatst een nieuwe reactie met `write_comment` en roept vervolgens `pin_comment` aan met de geretourneerde reactie-ID. Bij latere runs op dezelfde thread instrueert de prompt om eerst `unpin_comment` aan te roepen op de eerdere samenvatting - het platform zelf dwingt **geen** regel af van één gepinde reactie per thread, dus als je de vorige samenvatting gepind laat staan, krijg je twee gepinde samenvattingen naast elkaar. Vink "Include parent comment and prior replies in the same thread" aan in [Contextopties](#context-options) zodat de agent de eerdere gepinde samenvatting kan zien.

### Aanbevolen toevoegingen vóór livegang

- **Vink "Include parent comment and prior replies in the same thread" aan** in [Contextopties](#context-options). Een summarizer zonder threadcontext is nutteloos.
- **Stem de regel voor minimale threadgrootte af.** "Minder dan 5 reacties" is de standaardinstelling van de prompt, maar in drukke communities is 10-20 passender. Bewerk de prompt direct.
- **Beperk tot specifieke URL-patronen** als je alleen samenvattingen wilt op lange artikelpagina's, niet op aankondigingen of productpagina's. Zie [Bereik: URL- en landinstellingsfilters](#scope-url-locale).
- **Houd kosten in de gaten.** Samenvatten is het meest token-intensieve template omdat het bij elke run de hele thread leest. Stel een strak [dagbudget](#budgets-overview) in voordat je overschakelt naar Ingeschakeld.

### Herhaalde samenvattingen voorkomen

De agent heeft toegang tot [`save_memory`](#tools-overview) en [`search_memory`](#tools-overview) - je kunt de prompt uitbreiden met de instructie om "summarized {thread urlId}"-notities vast te leggen en hiernaar te zoeken voordat er opnieuw wordt geplaatst. Geheugen wordt gedeeld tussen alle agents in je tenant.