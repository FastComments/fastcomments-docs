**Dry Run** je bezbednosni režim u kojem svaki novi agent počinje. Agent se izvršava od početka do kraja osim dela u kome utiče na vašu zajednicu.

### What runs in Dry Run

- Okidači se pokreću normalno.
- Prompt agenta, [smernice zajednice](#community-guidelines) i [kontekst](#context-options) se sastavljaju.
- LLM se poziva.
- Model bira pozive alata i daje opravdanja + ocene pouzdanosti.
- Izvršavanje se beleži sa značkom **Dry Run** kako bi se jasno razlikovalo od živih izvršavanja.

### What does not run in Dry Run

- Nijedan komentar nije objavljen, nijedan glas nije dat, nijedan komentar nije zakačen/odkačen/zaključan/otključan.
- Nijedan komentar nije označen kao spam, odobren ili pregledan.
- Nijedan korisnik nije zabranjen, upozoren, niti nagrađen značkom.
- Nijedan email nije poslat.
- Ništa se ne upisuje u memoriju. (Da — uključujući memoriju. Dry-run agenti ne mogu zatrovati zajednički prostor za memoriju hipotetičkim odlukama.)
- Ne pokreću se webhooks za radnje alata. (Webhooks na nivou okidača `trigger.succeeded` / `trigger.failed` i dalje se pokreću i payload uključuje `wasDryRun: true`. Pogledajte [Podaci webhook-a](#webhook-payloads).)

### What it costs

Dry Run pokreće **isti LLM poziv** koji bi izvršavanje u stanju **Enabled** pokrenulo. Tokeni se naplaćuju, primenjuju se [ograničenja budžeta](#budgets-overview), i izvršavanja se računaju u dnevne/mesečne limite po agentu i po tenant-u.

Ta cena je cena za veran pregled. Režim „preskoči LLM poziv“ ne bi vam dao nikakav signal o tome kako bi se agent ponašao.

### Reading dry-run results

U [Run History](#run-history), dry-run izvršavanja su označena značkom **Dry Run** u koloni statusa. Radnje unutar svakog izvršavanja izgledaju identično kao žive radnje — isti naziv alata, isti argumenti, ista opravdanja i ocene pouzdanosti — osim što nijedna od njih nije zaista izvršena.

Stranica [Analytics](#analytics-page) razlaže "dry-run vs live" izvršavanja po mesecima tako da možete videti koliko je vašeg troška tokena otišlo na posmatranje.

### Switching out of Dry Run

Izmenite agenta i promenite **Status** u **Enabled**. Sledeći okidač će se pokrenuti uživo.

Takođe možete prebaciti i u suprotnom smeru — sa **Enabled** nazad na **Dry Run** — ako agent počne da radi stvari koje vam se ne dopadaju. Nema kazne.

### Replays force Dry Run

Funkcija [Test Runs (Replays)](#test-runs-replays) pokreće agenta protiv istorijskih komentara **uvek u dry-run režimu**, bez obzira na sačuvani status agenta. Replay-i ne mogu preduzimati stvarne radnje na prethodnim komentarima. To je namerno — replay je alat za pregled, a ne alat za moderaciju.