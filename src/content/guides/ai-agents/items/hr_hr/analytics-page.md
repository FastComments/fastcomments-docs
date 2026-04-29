Analitika je nadzorna ploča za više agenata. Dostupna je sa stranice AI agenata putem kartice **Analitika** (na razini tenanta) ili po agentu putem gumba **Analitika** u retku svakog agenta.

### Filtriranje

Padajući izbornik pri vrhu - **Svi agenti** ili određeni agent. Ostatak stranice prilagođava se odabiru.

### Korištenje proračuna

Četiri trake napretka koje prikazuju potrošnju u tekućem razdoblju u odnosu na limit:

- **Agent danas** (kada je filtrirano na određenog agenta) - dnevni limit po agentu.
- **Agent ovaj mjesec** - mjesečni limit po agentu.
- **Račun danas** - dnevni limit tenanta.
- **Račun ovaj mjesec** - mjesečni limit tenanta.

Kad limit nije postavljen, traka prikazuje "(no cap set)" i prikazuje sirovu potrošnju.

### Dnevni trošak (posljednjih 30 dana)

Tablica dnevnih troškova u valuti vašeg tenanta za odabrani opseg. Korisno za uočavanje:

- **Nagla povećanja troškova** - obično zbog nekontrolirane petlje ili viralnog komentara koji aktivira puno okidača.
- **Postupno povećanje troškova** - postupno povećanje dnevnih troškova kako vaša zajednica raste.

### Poduzete radnje

Razrada tipova radnji tijekom tekućeg mjeseca - "Wrote a comment: 47", "Marked a comment as spam: 12" i tako dalje. Korisno za provjeru da agent radi ono što ste očekivali.

### Preskočeni okidači (ovaj mjesec)

Brojke grupirane po [drop reason](#drop-reasons):

- Preko dnevnog limita agenta / mjesečnog limita agenta / dnevnog limita računa / mjesečnog limita računa.
- Ograničeno brzinom.
- Zasićenje konkurentnosti.

Ako vidite ovdje propuštanja, vaš agent dostiže proračunski ili ograničenje brzine i propušta okidače na kojima bi inače radio. Pogledajte [Drop Reasons](#drop-reasons).

### Dry-run vs uživo (ovaj mjesec)

- **Enabled runs** - broj izvođenja koja su poduzela stvarne radnje ovog mjeseca.
- **Dry runs** - broj izvođenja u dry-run modu ovog mjeseca.

Korisna informacija za podešavanje: potpuno novi agent koji još nije promoviran u Enabled prikazivat će samo dry-runove. Agent u Enabled statusu s nultim brojevima u ovom dijelu stoji u mirovanju - ili se ne aktivira, ili je izvan opsega, ili njegovi okidači nisu ispravno konfigurirani.

### Najskuplji agenti po mjesečnom trošku

Kada je filter **Svi agenti**, stranica navodi agente poredane po trošku od početka mjeseca do danas. Uočavanje najskupljeg agenta je prvi korak u optimizaciji troškova - obično je rješenje 'sužavanje njegovih [opcija konteksta](#context-options)' ili 'smanjenje njegovog [ograničenja proračuna](#budgets-overview)'.

### Agenti na ili blizu svog limita

Razrada po agentu za agente čija je potrošnja u tekućem razdoblju pri ili blizu njihovih ograničenja po agentu:

- **blizu limita** - iznad konfigurabilnog postotka limita.
- **preko limita** - zapravo ograničeno, s `{count} dropped` okidačima u tom razdoblju.

Kliknite na agenta iz ove tablice da podignete limit, suzite opseg ili ga pauzirate.

### Sažetak računa

Kada je filter **Svi agenti**:

- **Okidači danas** - broj.
- **Okidači ovaj mjesec** - broj.
- Za svaki: sufiks `dropped` koji pokazuje koliko je preskočeno.

### Valuta

Sve novčane vrijednosti prikazane su u valuti vašeg tenanta.

### Što ova stranica ne radi

- Ne prikazuje **raščlambu troškova po akciji** - te su na [Pregled detalja pokretanja](#run-detail-view).
- Ne prikazuje **transkripte** ili **LLM odgovore**.
- Ne omogućuje poduzimanje radnji nad agentima - uređivanje, pauziranje, brisanje rade se iz popisa agenata / stranice za uređivanje.