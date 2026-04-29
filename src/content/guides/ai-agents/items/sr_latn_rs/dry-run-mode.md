**Dry Run** je bezbednosni režim u kojem svaki novi agent započinje.

### What runs in Dry Run

- Okidači (triggers) se aktiviraju normalno.
- Prompt agenta, [smernice zajednice](#community-guidelines) i [kontekst](#context-options) se formiraju.
- Poziva se LLM.
- Model bira pozive alata i daje opravdanja + ocene poverenja.
- Izvršavanje se beleži sa bedžom **Dry Run** kako bi se jasno razlikovalo od live izvršavanja.

### What does not run in Dry Run

- Nijedan komentar nije objavljen, nijedno glasanje nije izvršeno, nijedan komentar nije pinned/unpinned/locked/unlocked.
- Nijedan komentar nije označen kao spam, odobren ili pregledan.
- Nijedan korisnik nije banovan, upozoren, niti mu je dodeljen bedž.
- Nijedan email nije poslat.
- Nije upisana nijedna memorija. (Da — uključujući memoriju. Dry-run agenti ne mogu zatrovati deljenu memoriju hipotetičkim odlukama.)
- Ne pokreću se webhooks za akcije alata. (Trigger-level `trigger.succeeded` / `trigger.failed` webhooks se i dalje pokreću i payload uključuje `wasDryRun: true`. Pogledajte [Webhook Payloads](#webhook-payloads).)

### What it costs

Dry Run pokretanja koriste **isti LLM poziv** kao i Enabled pokretanja. Tokeni se naplaćuju, primenjuju se [ograničenja budžeta](#budgets-overview), i izvršavanja se računaju u dnevne/mesečne limite po agentu i po tenant-u.

Ta cena je cena za dobijanje verne pretpreglede. Režim „preskoči LLM poziv“ ne bi vam dao nikakav signal o tome kako bi se agent ponašao.

### Reading dry-run results

U [Run History](#run-history), dry-run izvršavanja su označena bedžom **Dry Run** u koloni statusa. Akcije unutar svakog izvršavanja izgledaju identično kao žive akcije — isto ime alata, isti argumenti, isto opravdanje i ocena poverenja — osim što nijedna od njih zapravo nije izvršena.

Stranica [Analytics page](#analytics-page) razlaže „dry-run vs live“ izvršavanja po mesecu tako da možete videti koliko je vašeg troška tokena otišlo na posmatranje.

### Switching out of Dry Run

Izmenite agenta i promenite **Status** u **Enabled**. Sledeći okidač će se pokrenuti uživo.

Takođe možete uraditi i obrnuto — Enabled nazad na Dry Run — ako agent počne da radi stvari koje vam se ne sviđaju. Nema kazne.

### Replays force Dry Run

Funkcija [Test Runs (Replays)](#test-runs-replays) pokreće agenta protiv istorijskih komentara **uvek u dry-run** režimu, bez obzira na sačuvani status agenta. Replays ne mogu preduzeti prave akcije nad prošlim komentarima. To je namerno — replay je alat za pretpregled, a ne alat za moderaciju.