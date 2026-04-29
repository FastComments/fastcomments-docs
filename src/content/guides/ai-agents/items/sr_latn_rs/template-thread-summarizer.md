**ID šablona:** `thread_summarizer`

Sažimač teme objavljuje neutralan, jednoparagrafni sažetak na kraju duge teme. Koristi 30-minutno odlaganje kako bi se tema smirila pre nego što agent pogleda.

Ugrađeni prompt naređuje agentu da ne daje uredničke komentare — ovo je od ključne važnosti. Bez toga model teži formulacijama poput "in my view" koje loše zvuče pod imenom vašeg naloga.

### Okidači

- **Novi komentar postavljen** (`COMMENT_ADD`).
- **Vreme odlaganja okidača**: 30 minuta (1800 sekundi). Pogledajte [Deferred Triggers](#trigger-deferred-delay).

30-minutno odlaganje znači da agent radi jednom, pola sata nakon što komentar stigne, u skladu sa tim kakva tema izgleda u tom trenutku. Nije u pitanju "sažimaj na svaki odgovor" — red odloženih okidača koalescira višestruke događaje novog komentara u istoj temi, ali ih ne de-duplikuje kroz zasebne intervale. Verovatno ćete želeti da **dodate prilagođeno pravilo u svoj prompt** poput "ne objavljuj novi sažetak ako je agent već sažeo ovu temu u poslednjih 24 sata" i oslonite se na kontekst plus agentove [memory tools](#tools-overview) da to sprovedu.

### Dozvoljeni alati

- [`write_comment`](#tools-overview) - postavlja sam sažetak.
- [`pin_comment`](#tools-overview) - prikvači sažetak kako bi čitaoci videli na vrhu teme.
- [`unpin_comment`](#tools-overview) - otkači prethodni sažetak istog agenta pre prikvačenja novog.

Sažimač ne može da moderira ili interaguje sa korisnicima.

### Prikvačivanje sažetka

Agent postavlja novi komentar pomoću `write_comment`, zatim poziva `pin_comment` sa vraćenim ID-jem komentara. Prilikom narednih pokretanja protiv iste teme, prompt ga upućuje da prvo pozove `unpin_comment` na svom prethodnom sažetku — sama platforma **ne** nameće pravilo jednog prikvačenog komentara po temi, tako da ostavljanje prethodnog sažetka prikvačenog rezultiraće sa dva prikvačena sažetka jedan pored drugog. Štickirajte "Include parent comment and prior replies in the same thread" u [Context Options](#context-options) kako bi agent mogao da vidi prethodni prikvačeni sažetak.

### Preporučeni dodaci pre objavljivanja

- **Štickirajte "Include parent comment and prior replies in the same thread"** u [Context Options](#context-options). Sažimač bez konteksta teme je beskoristan.
- **Podesite pravilo minimalne veličine teme.** "Manje od 5 komentara" je podrazumevano u promptu, ali u prometnim zajednicama 10–20 je prikladnije. Uredite prompt direktno.
- **Ograničite na specifične URL obrasce** ako želite sažetke samo na stranicama dugog formata, a ne na najavama ili stranicama proizvoda. Pogledajte [Scope: URL and Locale Filters](#scope-url-locale).
- **Pratite troškove.** Sažimanje troši najviše tokena jer čita celu temu pri svakom pokretanju. Postavite strogi [daily budget](#budgets-overview) pre nego što prebacite na Omogućeno.

### Izbegavanje ponovljenih sažetaka

Agent ima pristup [`save_memory`](#tools-overview) i [`search_memory`](#tools-overview) - možete proširiti prompt da ga naložite da zabeleži beleške "summarized {thread urlId}" i proveri ih pre ponovnog objavljivanja. Memorija se deli između svih agenata u vašem tenant-u.