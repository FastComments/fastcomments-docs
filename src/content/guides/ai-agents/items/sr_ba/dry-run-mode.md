**Probni režim** je sigurnosni način u kojem svaki novi agent počinje. Agent radi od početka do kraja osim dijela u kojem dodiruje vašu zajednicu.

### Šta radi u Probnom režimu

- Okidači se normalno aktiviraju.
- Prompt agenta, [smjernice zajednice](#community-guidelines), i [kontekst](#context-options) se sastavljaju.
- LLM se poziva.
- Model bira pozive alata i daje opravdanja + ocjene povjerenja.
- Izvršavanje se evidentira oznakom **Probni režim** kako bi se jasno razlikovalo od uživo izvršavanja.

### Šta se ne izvršava u Probnom režimu

- Nijedan komentar se ne objavljuje, nijedan glas se ne daje, nijedan komentar se ne prikači/otkači/zaključa/otključa.
- Nijedan komentar nije označen kao spam, odobren ili pregledan.
- Nijedan korisnik nije banovan, upozoren ili nagrađen značkom.
- Nijedan e-mail se ne šalje.
- Nijedna memorija se ne zapisuje. (Da — uključujući memoriju. Agenti u probnom režimu ne mogu zatrovati zajednički skup memorije hipotetičkim odlukama.)
- Ne pokreću se webhooks za akcije alata. (Webhookovi na nivou okidača `trigger.succeeded` / `trigger.failed` i dalje se pokreću i payload sadrži `wasDryRun: true`. Vidi [Podaci Webhook-a](#webhook-payloads).)

### Koliko košta

Probni režim izvršava **istu LLM poziv** kao i izvršavanje u statusu **Omogućeno**. Tokeni se naplaćuju, [ograničenja budžeta](#budgets-overview) se primjenjuju, i izvršavanja se računaju protiv dnevnih/mjesečnih ograničenja po agentu i po tenant-u.

Taj trošak je cijena dobivanja vjernog pregleda. Režim "preskoči LLM poziv" vam ne bi dao nikakav signal o tome kako bi se agent ponašao.

### Pregled rezultata probnog režima

U [Povijest izvršavanja](#run-history), izvršavanja u probnom režimu su označena oznakom **Probni režim** u koloni statusa. Akcije unutar svakog izvršavanja izgledaju identično kao akcije uživo - isto ime alata, isti argumenti, ista opravdanja i povjerenje - osim što se nijedna od tih akcija nije stvarno dogodila.

Stranica [Analitike](#analytics-page) razlaže "probni režim naspram uživo" izvršavanja po mjesecu kako biste vidjeli koliko je vaših tokena potrošeno na promatranje.

### Prebacivanje iz Probnog režima

Uredite agenta i promijenite **Status** u **Omogućeno**. Sljedeći okidač će se pokrenuti uživo.

Možete također prebaciti i obratno - iz **Omogućeno** nazad u **Probni režim** - ako agent počne raditi stvari koje vam se ne sviđaju. Nema kazne.

### Reprodukcije prisiljavaju Probni režim

Funkcija [Testna izvršavanja (Reprodukcije)](#test-runs-replays) pokreće agenta protiv historijskih komentara **uvijek u probnom režimu**, bez obzira na spremljeni status agenta. Reprodukcije ne mogu preduzimati stvarne akcije nad prošlim komentarima. To je namjerno - reprodukcija je alat za pregled, a ne alat za moderaciju.