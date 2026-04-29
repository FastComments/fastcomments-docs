**ID predloška:** `thread_summarizer`

Thread Summarizer objavljuje neutralan, jednoparagrafski sažetak na kraju duge niti. Koristi odgodu od 30 minuta kako bi se nit smirila prije nego što je agent pregleda.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt predloška Sažimač niti'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

Uputa "do not editorialize" je ključna. Bez nje model teži formulacijama tipa "in my view" koje loše zvuče pod prikaznim imenom vašeg računa.

### Okidači

- **Novi komentar objavljen** (`COMMENT_ADD`).
- **Kašnjenje okidača**: 30 minuta (1800 sekundi). Vidi [Deferred Triggers](#trigger-deferred-delay).

Odgoda od 30 minuta znači da agent radi jednom, pola sata nakon što stigne komentar, na temelju stanja niti u tom trenutku. To nije "sažmi pri svakom odgovoru" — red za odgođene okidače koalescira više događaja novog komentara na istoj niti, ali ih ne deduplikira preko zasebnih prozora. Vjerojatno ćete htjeti **dodati prilagoavljeno pravilo u svom promptu** kao što je "ne objavljuj novi sažetak ako je agent već sažeo ovu nit u zadnjih 24 sata" i osloniti se na kontekst plus agentove [alate za memoriju](#tools-overview) da to provjeri.

### Dopušteni alati

- [`write_comment`](#tools-overview) - objavljuje sam sažetak.
- [`pin_comment`](#tools-overview) - prikvači sažetak tako da ga čitatelji vide na vrhu niti.
- [`unpin_comment`](#tools-overview) - ukloni prikvačeni prethodni sažetak istog agenta prije prikvačivanja novog.

Sažimač ne može moderirati niti komunicirati s korisnicima.

### Prikvačivanje sažetka

Agent objavi novi komentar pomoću `write_comment`, zatim pozove `pin_comment` s vraćenim ID-jem komentara. Kod narednih pokretanja na istoj niti, prompt ga upućuje da prvo pozove `unpin_comment` za svoj prethodni sažetak — platforma sama po sebi ne provodi pravilo o samo jednom prikvačenom komentaru po niti, pa će ostavljanje prethodnog sažetka prikvačenog rezultirati dvama prikvačenim sažecima jedan uz drugoga. Označite "Include parent comment and prior replies in the same thread" u [Context Options](#context-options) kako bi agent mogao vidjeti prethodni prikvačeni sažetak.

### Preporučene dopune prije pokretanja

- **Označite "Include parent comment and prior replies in the same thread"** u [Context Options](#context-options). Sažimač bez konteksta niti je beskoristan.
- **Prilagodite pravilo o minimalnoj veličini niti.** "Fewer than 5 comments" je zadana vrijednost u promptu, ali u prometnijim zajednicama prikladnije je 10–20. Uredite prompt izravno.
- **Ograničite na određene obrasce URL-a** ako želite sažetke samo na stranicama dugog oblika, a ne na objavama ili stranicama proizvoda. Vidi [Scope: URL and Locale Filters](#scope-url-locale).
- **Pazite na troškove.** Sažimanje troši najviše tokena jer čita cijelu nit pri svakom pokretanju. Postavite strogi [dnevni proračun](#budgets-overview) prije nego što prebacite na Omogućeno.

### Izbjegavanje ponovljenih sažetaka

Agent ima pristup [`save_memory`](#tools-overview) i [`search_memory`](#tools-overview) - možete proširiti prompt da ga uputite da zapiše bilješke poput "sažeto {thread urlId}" i provjeri ih prije ponovnog objavljivanja. Memorija je dijeljena među svim agentima u vašem tenant-u.