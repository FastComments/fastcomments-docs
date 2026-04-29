**ID predloška:** `thread_summarizer`

Sažimatelj niti objavljuje neutralan, jednogodišnji sažetak u jednom odlomku na kraju duge niti. Koristi odgodu od 30 minuta kako bi se nit smirila prije nego što je agent pregleda.

Ugrađeni prompt upućuje agenta da ne uređuje tekst (ne bude urednički) — to je ključna uputa. Bez nje model teži formulacijama poput "in my view" koje loše zvuče pod prikazanim imenom vašeg računa.

### Okidači

- **Objavljen novi komentar** (`COMMENT_ADD`).
- **Odgoda okidača**: 30 minuta (1800 sekundi). Vidi [Odgođeni okidači](#trigger-deferred-delay).

Odgoda od 30 minuta znači da se agent pokreće jednom, pola sata nakon što komentar stigne, koristeći stanje niti u tom trenutku. To nije "sažmi na svaki odgovor" — red odgođenih okidača spaja više događaja "novi komentar" na istoj niti, ali ih ne uklanja kao duplikate preko zasebnih vremenskih prozora. Vjerojatno ćete htjeti **dodati prilagođeno pravilo u vaš prompt** poput "ne objavljuj novi sažetak ako je agent već sažeo ovu nit u posljednjih 24 sata" i osloniti se na kontekst plus agentove [alate za memoriju](#tools-overview) da to provede.

### Dozvoljeni alati

- [`write_comment`](#tools-overview) - objavljuje sam sažetak.
- [`pin_comment`](#tools-overview) - prikvači sažetak tako da ga čitatelji vide na vrhu niti.
- [`unpin_comment`](#tools-overview) - uklanja prikvačeni status prethodnog sažetka istog agenta prije prikvačenja novog.

Sažimatelj ne može moderirati niti komunicirati s korisnicima.

### Prikvačivanje sažetka

Agent objavljuje novi komentar pomoću `write_comment`, zatim poziva `pin_comment` s vraćenim ID-om komentara. Pri naknadnim pokretanjima za istu nit, prompt ga upućuje da prvo pozove `unpin_comment` za svoj prethodni sažetak — sama platforma NE provodi pravilo o jednom prikvačenom komentaru po niti, pa ostavljanje prethodnog sažetka prikvačenog rezultira dvama prikvačenim sažecima jedan do drugoga. Označite "Uključi roditeljski komentar i prethodne odgovore u istoj niti" u [Opcijama konteksta](#context-options) kako bi agent mogao vidjeti prethodni prikvačeni sažetak.

### Preporučene dopune prije puštanja u rad

- **Označite "Uključi roditeljski komentar i prethodne odgovore u istoj niti"** u [Opcijama konteksta](#context-options). Sažimatelj bez konteksta niti je beskoristan.
- **Prilagodite pravilo o minimalnoj veličini niti.** 'Manje od 5 komentara' je zadana vrijednost prompta, ali u prometnim zajednicama prikladnije je 10–20. Uredite prompt izravno.
- **Ograničite na određene uzorke URL-a** ako želite sažetke samo na stranicama dugog oblikа, a ne na objavama ili stranicama proizvoda. Vidi [Opseg: filteri URL-a i lokaliteta](#scope-url-locale).
- **Pazite na troškove.** Sažimanje je najzahtjevniji predložak po tokenima jer čita cijelu nit pri svakom pokretanju. Postavite strogi [dnevni budžet](#budgets-overview) prije nego što ga postavite na Omogućeno.

### Izbjegavanje ponovljenih sažetaka

Agent ima pristup [`save_memory`](#tools-overview) i [`search_memory`](#tools-overview) — možete proširiti prompt da ga uputite da zabilježi bilješke "summarized {thread urlId}" i provjeri ih prije ponovne objave. Memorija je dijeljena među svim agentima u vašem tenantu.