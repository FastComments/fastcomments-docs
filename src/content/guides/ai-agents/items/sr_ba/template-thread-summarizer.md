**Template ID:** `thread_summarizer`

Thread Summarizer objavljuje neutralan, jednogodišnji paragraf sažetka na kraju duže teme. Koristi 30-minutno odlaganje tako da se tema može smiriti prije nego što je agent pregleda.

Ugrađeni prompt nalaže agentu da ne uređuje/da ne dodaje uređivačke komentare — ovo je od ključne važnosti. Bez toga model teži formulacijama tipa "po mom mišljenju" koje loše zvuče pod prikaznim imenom vašeg naloga.

### Okidači

- **Novi komentar objavljen** (`COMMENT_ADD`).
- **Kašnjenje okidača**: 30 minuta (1800 sekundi). Vidi [Deferred Triggers](#trigger-deferred-delay).

30-minutno odlaganje znači da agent pokreće jednom, pola sata nakon što komentar stigne, u odnosu na to kako tema izgleda u tom trenutku. To nije "sumirati pri svakom odgovoru" — red za odložene okidače spaja više događaja novog komentara na istoj temi, ali ih ne de-duplicira kroz odvojene vremenske prozore. Vjerovatno ćete htjeti **dodati prilagođeno pravilo u svom promptu** poput "ne objavljuj novi sažetak ako je agent već sažeo ovu temu u posljednjih 24 sata" i osloniti se na kontekst plus agentove [alate za memoriju](#tools-overview) da to provede.

### Dozvoljeni alati

- [`write_comment`](#tools-overview) - objavljuje sam sažetak.
- [`pin_comment`](#tools-overview) - prikvači sažetak kako bi čitaoci vidjeli na vrhu teme.
- [`unpin_comment`](#tools-overview) - uklanja prikvačenje prethodnog sažetka istog agenta prije prikvačivanja novog.

Sažimač ne može moderirati niti komunicirati s korisnicima.

### Prikvačivanje sažetka

Agent objavljuje novi komentar sa `write_comment`, zatim poziva `pin_comment` sa vraćenim ID-jem komentara. Pri narednim pokretanjima protiv iste teme, prompt nalaže da prvo pozove `unpin_comment` na svom prethodnom sažetku — sama platforma ne nameće pravilo o jedinom prikvačenom komentaru po temi, pa ostavljanje prethodnog sažetka prikvačenog rezultiraće sa dva prikvačena sažetka jedan pored drugog. Označite "Include parent comment and prior replies in the same thread" u [Context Options](#context-options) tako da agent može vidjeti prethodni prikvačeni sažetak.

### Preporučeni dodaci prije puštanja u rad

- **Označite "Include parent comment and prior replies in the same thread"** u [Context Options](#context-options). Sažimač bez konteksta teme je beskoristan.
- **Podesite pravilo minimalne veličine teme.** "Fewer than 5 comments" je zadani prompt, ali u prometnim zajednicama 10–20 je prikladnije. Uredite prompt direktno.
- **Ograničite na specifične obrasce URL-a** ako želite sažetke samo na stranicama dužeg sadržaja, a ne na objavama ili stranicama proizvoda. Vidi [Scope: URL and Locale Filters](#scope-url-locale).
- **Pratite troškove.** Sažimanje troši najviše tokena jer čita cijelu temu pri svakom pokretanju. Postavite strogi [dnevni budžet](#budgets-overview) prije nego što prebacite na Enabled.

### Izbjegavanje ponovljenih sažetaka

Agent ima pristup [`save_memory`](#tools-overview) i [`search_memory`](#tools-overview) - možete proširiti prompt da mu naložite da zabilježi bilješke poput 'sumirano {thread urlId}' i provjeri ih prije ponovnog objavljivanja. Memorija se dijeli među svim agentima u vašem tenant-u.