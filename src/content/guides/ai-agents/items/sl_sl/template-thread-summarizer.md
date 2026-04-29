---
**ID predloge:** `thread_summarizer`

Povzemalnik nití objavi nevtralen povzetek v enem odstavku na koncu dolge niti. Uporablja 30-minutni odlog, da se nit umiri, preden agent nanjo pogleda.

Vgrajeni poziv agentu natančno naroča, naj ne ureja besedila po svoje — to je ključno. Brez tega se model nagiba k okvirjenju z "po mojem mnenju", kar pri prikazu pod imenom vašega računa slabo izpade.

### Sprožilci

- **Nov komentar objavljen** (`COMMENT_ADD`).
- **Zamuda sprožilca**: 30 minut (1800 sekund). Glejte [Odloženi sprožilci](#trigger-deferred-delay).

30-minutna zamuda pomeni, da agent deluje enkrat, pol ure po pristanku komentarja, glede na stanje niti v tistem trenutku. Ne pomeni "povzemaj ob vsakem odgovoru" — čakalna vrsta odloženih sprožilcev združi več dogodkov novega komentarja na isti niti, vendar jih ne deduplikira čez ločena časovna okna. Verjetno boste želeli **dodati lastno pravilo v vaš poziv** kot npr. "ne objavljajte novega povzetka, če je agent že povzel to nit v zadnjih 24 urah" in se zanašati na kontekst ter agentova [orodja za pomnilnik](#tools-overview), da to uveljavi.

### Dovoljena orodja

- [`write_comment`](#tools-overview) - objavi sam povzetek.
- [`pin_comment`](#tools-overview) - pripne povzetek, da ga bralci vidijo na vrhu niti.
- [`unpin_comment`](#tools-overview) - odpnese prejšnji povzetek istega agenta pred pripenjanjem novega.

Povzemalnik ne more moderirati ali sodelovati z uporabniki.

### Pripenjanje povzetka

Agent najprej objavi nov komentar z `write_comment`, nato pokliče `pin_comment` s pridobljenim ID-jem komentarja. Pri naslednjih izvedbah na isti niti poziv naroča, naj najprej pokliče `unpin_comment` za svoj prejšnji povzetek — platforma sama ne uveljavlja pravila enega samega pripetega komentarja na nit, zato bo puščanje prejšnjega povzetka pripetega povzročilo dva pripeta povzetka drug ob drugem. V [Možnostih konteksta](#context-options) obkljukajte "Include parent comment and prior replies in the same thread", da bo agent videl prejšnji pripeti povzetek.

### Priporočene dodatke pred vklopom v živo

- **Obkljukajte "Include parent comment and prior replies in the same thread"** v [Možnostih konteksta](#context-options). Povzemalnik brez konteksta niti je neuporaben.
- **Prilagodite pravilo o minimalni velikosti niti.** Privzeto je v pozivu "Fewer than 5 comments", vendar je v zaposlenih skupnostih bolj primerno 10–20. Uredite poziv neposredno.
- **Omejite na določene vzorce URL-jev**, če želite povzetke le na straneh z dolgimi vsebinami, ne pa na obvestilih ali produktnih straneh. Glejte [Obseg: filtri URL in lokalizacije](#scope-url-locale).
- **Spremljajte stroške.** Povzemanje porabi največ žetonov, ker ob vsakem zagonu prebere celotno nit. Pred preklopom na Omogočeno nastavite strogo [dnevno proračun](#budgets-overview).

### Izogibanje ponavljajočim se povzetkom

Agent ima dostop do [`save_memory`](#tools-overview) in [`search_memory`](#tools-overview) - v poziv lahko dodate navodilo, naj zapiše opombe "summarized {thread urlId}" in jih preveri, preden ponovno objavi. Pomnilnik je deljen med vsemi agenti v vašem najemu.
---