**Template ID:** `thread_summarizer`

Sažimač teme (Thread Summarizer) objavljuje neutralan, jednoparagrafski sažetak na kraju duže teme. Koristi odlaganje od 30 minuta kako bi se tema smirila pre nego što agent pogleda.

### Built-in initial prompt

[inline-code-attrs-start title = 'Početni prompt šablona za sažetak teme'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

Uputstvo „ne uređivački“ (do not editorialize) je ključna smernica. Bez njega model ima tendenciju da ide ka formulacijama tipa „po mom mišljenju” koje loše zvuče pod prikaznim imenom vašeg naloga.

### Triggers

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [Deferred Triggers](#trigger-deferred-delay).

Kašnjenje od 30 minuta znači da agent radi jednom, pola sata nakon što je komentar postavljen, na osnovu stanja teme u tom trenutku. To nije „sažmi svaki odgovor“ — red za odložene okidače koalescira više događaja novog komentara u istoj temi, ali ih ne dedupira preko odvojenih vremenskih prozora. Verovatno ćete želeti da **dodate prilagođeno pravilo u vaš prompt** kao što je "ne postavljaj novi sažetak ako je agent već sažeo ovu temu u poslednjih 24 sata" i oslonite se na kontekst plus agentove [alatke za memoriju](#tools-overview) da to sprovede.

### Allowed tools

- [`write_comment`](#tools-overview) - postavlja sam sažetak.
- [`pin_comment`](#tools-overview) - fiksira sažetak tako da ga čitaoci vide na vrhu teme.
- [`unpin_comment`](#tools-overview) - uklanja fiksiranje prethodnog sažetka istog agenta pre fiksiranja novog.

Sažimač ne može da moderira niti da komunicira sa korisnicima.

### Pinning the summary

Agent objavljuje novi komentar pomoću `write_comment`, zatim poziva `pin_comment` sa vraćenim ID-jem komentara. Prilikom narednih pokretanja na istoj temi, prompt ga instruiše da prvo pozove `unpin_comment` na svom prethodnom sažetku — platforma sama po sebi NE nameće pravilo o jednom fiksiranom komentaru po temi, tako da ostavljanje prethodnog sažetka fiksiranim može rezultirati sa dva fiksirana sažetka jedan pored drugog. Označite "Include parent comment and prior replies in the same thread" u [Context Options](#context-options) tako da agent može da vidi prethodni fiksirani sažetak.

### Recommended additions before going live

- **Označite "Include parent comment and prior replies in the same thread"** u [Context Options](#context-options). Sažimač bez konteksta teme je beskoristan.
- **Podesite pravilo minimalne veličine teme.** "Manje od 5 komentara" je podrazumevana vrednost u promptu, ali u prometnim zajednicama 10–20 je primerenije. Izmenite prompt direktno.
- **Ograničite na određene URL obrasce** ako želite sažetke samo na stranicama dugog formata, a ne na obaveštenjima ili stranicama proizvoda. Vidi [Scope: URL and Locale Filters](#scope-url-locale).
- **Pazite na troškove.** Sažimanje troši najviše tokena jer čita celu temu pri svakom pokretanju. Pre nego što omogućite, postavite strogi [dnevni budžet](#budgets-overview).

### Avoiding repeat summaries

Agent ima pristup [`save_memory`](#tools-overview) i [`search_memory`](#tools-overview) - možete proširiti prompt da ga uputite da beleži zapise „summarized {thread urlId}“ i proverava pre ponovnog objavljivanja. Memorija je zajednička za sve agente u vašem tenant-u.