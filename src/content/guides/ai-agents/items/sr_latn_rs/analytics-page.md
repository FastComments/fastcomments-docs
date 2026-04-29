Analitika je centralna kontrolna tabla za više agenata. Pristupačna sa stranice AI agenata preko kartice **Analitika** (za ceo tenant) ili po agentu putem dugmeta **Analitika** u redu svakog agenta.

### Filtriranje

Padajući meni na vrhu - **Svi agenti** ili određeni agent. Ostatak stranice se preusmerava u skladu sa tim.

### Korišćenje budžeta

Četiri trake napretka koje prikazuju potrošnju za tekući period u odnosu na limit:

- **Agent danas** (kada je filtrirano na određenog agenta) - dnevni limit agenta.
- **Agent ovog meseca** - mesečni limit agenta.
- **Nalog danas** - dnevni limit tenanta.
- **Nalog ovog meseca** - mesečni limit tenanta.

Kada limit nije postavljen, traka prikazuje "(no cap set)" i pokazuje sirovu potrošnju.

### Dnevni trošak (poslednjih 30 dana)

Tabela dnevnog troška u valuti vašeg tenanta za izabrani opseg. Korisno za uočavanje:

- **Iznenadnih skokova troškova** - obično od beskonacne petlje ili viralnog komentara koji širi okidače.
- **Pomaka u troškovima** - postepeno povećanje dnevnog troška kako vaša zajednica raste.

### Preduzete akcije

Raspodela tipova akcija tokom tekućeg meseca - "Napisao komentar: 47", "Označio komentar kao spam: 12" i slično. Korisno za proveru da agent radi ono što očekujete.

### Preskočeni okidači (ovaj mesec)

Brojevi grupisani po [Razlozi za odbacivanje](#drop-reasons):

- Zbog prekoračenja dnevnog / mesečnog limita agenta ili dnevnog / mesečnog limita naloga.
- Ograničen saobraćaj (rate-limited).
- Zasićena konkurentnost.

Ako vidite padove ovde, vaš agent dostiže budžet ili ograničenje brzine i propušta okidače koje bi inače izvršio. Pogledajte [Razloge za odbacivanje](#drop-reasons).

### Dry-run vs uživo (ovaj mesec)

- **Omogućena pokretanja** - broj pokretanja koja su preduzela stvarne akcije ovog meseca.
- **Dry-run pokretanja** - broj pokretanja u dry-run modu ovog meseca.

Korisni signal za podešavanje: potpuno novi agent koji još nije unapređen u Omogućen prikazaće samo dry-run pokretanja. Agent koji je Omogućen, a ima nula u ovoj sekciji, miruje — ili nije okidan, ili je izuzet iz opsega, ili njegovi okidači nisu pravilno konfigurirani.

### Agenti po mesečnom trošku

Kada je filter **Svi agenti**, stranica prikazuje agente rangirane po trošku od početka meseca do danas. Pronalazak vašeg najskupljeg agenta je prvi korak u optimizaciji troškova - obično je rešenje "ostrimiti njegove [opcije konteksta](#context-options)" ili "smanjiti njegov [granični budžet](#budgets-overview)".

### Agenti koji su na ili blizu limita

Raspodela po agentu za agente čija je potrošnja na ili blizu njihovih per-agent limita u tekućem periodu:

- **blizu limita** - preko konfigurisane procentualne vrednosti limita.
- **preko limita** - zapravo ograničen, sa `{count} dropped` okidača u tom periodu.

Kliknite na agenta iz ove tabele da povećate limit, suzite opseg ili ga pauzirate.

### Pregled naloga

Kada je filter **Svi agenti**:

- **Okidači danas** - broj.
- **Okidači ovog meseca** - broj.
- Za oba: sufiks `dropped` koji pokazuje koliko je bilo preskočeno.

### Valuta

Sve novčane vrednosti su prikazane u valuti vašeg tenanta.

### Šta ova stranica ne radi

- Ne prikazuje **raspodelu troškova po akciji** - to je na [Pregledu detalja pokretanja](#run-detail-view).
- Ne prikazuje **transkripte** ili **LLM odgovore**.
- Ne omogućava da preduzmete radnje nad agentima - uređivanje, pauziranje, brisanje se rade iz liste agenata / stranice za uređivanje.