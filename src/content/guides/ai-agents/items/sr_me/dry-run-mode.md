**Dry Run** je sigurnosni režim u kojem svaki novi agent započinje.

### Šta se izvršava u Dry Run

- Okidači se aktiviraju normalno.
- Agentov prompt, [smernice zajednice](#community-guidelines) i [kontekst](#context-options) se sastavljaju.
- Poziva se LLM.
- Model bira pozive alata i daje opravdanja + ocene poverenja.
- Pokretanje se beleži sa značkom **Dry Run** tako da je jasno razlikovano od izvršavanja uživo.

### Šta se ne izvršava u Dry Run

- Nijedan komentar nije objavljen, nijedan glas nije dat, nijedan komentar nije prikvačen/otkačen/zaključan/otključan.
- Nijedan komentar nije označen kao spam, odobren ili pregledan.
- Nijedan korisnik nije banovan, upozoren, niti mu je dodeljena značka.
- Ne šalje se nijedan email.
- Ne upisuje se nijakva memorija. (Da — uključujući memoriju. Agenti u režimu Dry Run ne mogu zatrovati zajednički memorijski prostor hipotetičkim odlukama.)
- Za akcije alata se ne aktiviraju webhookovi. (Webhookovi na nivou okidača `trigger.succeeded` / `trigger.failed` i dalje se aktiviraju i payload uključuje `wasDryRun: true`. Pogledajte [Webhook Payloads](#webhook-payloads).)

### Koliko to košta

Dry Run izvršava **isti LLM poziv** koji bi izvršavanje sa statusom Enabled izvelo. Tokeni se naplaćuju, primenjuju se [ograničenja budžeta](#budgets-overview), i pokretanja se računaju u dnevne/mesečne limite po agentu i po zakupcu.

Taj trošak je cena za dobijanje vernog pregleda. Režim "preskoči LLM poziv" ne bi dao nikakav signal o tome kako bi se agent ponašao.

### Pregled rezultata Dry Run

U [Istorija pokretanja](#run-history), izvršavanja u režimu dry-run su označena značkom **Dry Run** u koloni statusa. Akcije unutar svakog pokretanja izgledaju identično kao akcije uživo — isti naziv alata, isti argumenti, ista opravdanja i ocene poverenja — osim što nijedna od njih nije zaista izvršena.

Stranica [Analitika](#analytics-page) razlaže mesečne "dry-run vs live" pokretanja tako da možete videti koliko vaše potrošnje tokena je otišlo na posmatranje.

### Prebacivanje iz Dry Run

Uredite agenta i promenite **Status** u **Enabled**. Sledeći okidač će se izvršiti uživo.

Takođe možete preći i u suprotnom smeru — iz Enabled nazad u Dry Run — ako agent počne raditi stvari koje vam se ne dopadaju. Nema kazne.

### Reprodukcije prisiljavaju Dry Run

Funkcija [Test Runs (Replays)](#test-runs-replays) pokreće agenta protiv istorijskih komentara **uvek u dry-run režimu**, bez obzira na sačuvani status agenta. Reprodukcije ne mogu preduzeti stvarne akcije na prošlim komentarima. To je namerno — reprodukcija je alat za pregled, a ne alat za moderaciju.