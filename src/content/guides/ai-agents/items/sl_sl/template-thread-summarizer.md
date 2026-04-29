**Template ID:** `thread_summarizer`

The Thread Summarizer posts a neutral, single-paragraph summary at the end of a long thread. It uses a 30-minute deferral so the thread can settle before the agent looks at it.

### Built-in initial prompt

[inline-code-attrs-start title = 'Začetni poziv predloge za povzemanje niti'; type='text' inline-code-attrs-end]
[inline-code-start]
Objavljate nevtralne povzetke niti. Ne povzemajte niti, ki imajo manj kot 5 komentarjev. Za daljše niti povzemi glavna stališča, nestrinjanja in odprta vprašanja v enem kratkem odstavku. Ne zavzemajte strani in ne dodajajte uredniških komentarjev. Po objavi povzetka ga pripnite. Če je vaš prejšnji povzetek na tej niti že pripet, ga najprej odpnite, preden pripnete novega.
[inline-code-end]

Navodilo "ne dodajajte uredniških komentarjev" je ključnega pomena. Brez njega se model nagiba k formulacijam, kot je "po mojem mnenju", kar pod prikaznim imenom vašega računa zveni slabo.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [Deferred Triggers](#trigger-deferred-delay).

The 30-minute delay means the agent runs once, half an hour after a comment lands, against whatever the thread looks like at that moment. It is not "summarize on every reply" - the deferred-trigger queue coalesces multiple new-comment events on the same thread, but does not de-duplicate them across separate windows. You will likely want to **add a custom rule in your prompt** like "do not post a new summary if the agent has already summarized this thread in the last 24 hours" and rely on context plus the agent's [memory tools](#tools-overview) to enforce it.

### Allowed tools

- [`write_comment`](#tools-overview) - objavi povzetek.
- [`pin_comment`](#tools-overview) - pripne povzetek, da ga bralci vidijo na vrhu niti.
- [`unpin_comment`](#tools-overview) - odpne predhodni povzetek istega agenta, preden pripne novega.

Povzemalec ne more moderirati ali komunicirati z uporabniki.

### Pinning the summary

Agent objavi nov komentar z `write_comment`, nato pokliče `pin_comment` z ID-jem vrnjenega komentarja. Pri ponovnem zagonu za isto nit poziv navede, naj najprej pokliče `unpin_comment` za svoj predhodni povzetek — platforma sama **ne** izvaja pravila o enem pripetem komentarju na nit, zato bo ob puščanju prejšnjega povzetka pripetega nastalo dva pripeta povzetka drug ob drugem. Označite "Vključi nadrejeni komentar in prejšnje odgovore v isti niti" v [Context Options](#context-options), da agent vidi prejšnji pripeti povzetek.

### Recommended additions before going live

- **Označite "Vključi nadrejeni komentar in prejšnje odgovore v isti niti"** v [Context Options](#context-options). Povzemalec brez konteksta niti je neuporaben.
- **Prilagodite pravilo minimalne velikosti niti.** "Manj kot 5 komentarjev" je privzeta nastavitev v pozivu, vendar je v živahnih skupnostih primernejše 10–20. Uredite poziv neposredno.
- **Omejite na določene vzorce URL-jev** če želite povzetke le na dolgotrajnih straneh, ne na obvestilih ali straneh izdelkov. Oglejte si [Scope: URL and Locale Filters](#scope-url-locale).
- **Spremljajte stroške.** Povzemanje porabi največ žetonov, ker ob vsakem zagonu prebere celotno nit. Pred preklopom na Enabled nastavite strog [dnevni proračun](#budgets-overview).

### Avoiding repeat summaries

The agent has access to [`save_memory`](#tools-overview) and [`search_memory`](#tools-overview) - you can extend the prompt to instruct it to record "summarized {thread urlId}" notes and check for them before posting again. Memory is shared across all agents in your tenant.