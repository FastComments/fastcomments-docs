**Dry Run** je bezbednosni režim u kojem svaki novi agent počinje. Agent radi od početka do kraja, osim dela gde dodiruje vašu zajednicu.

### Šta se izvršava u Dry Run režimu

- Okidači (triggers) se aktiviraju normalno.
- Prompt agenta, [smernice zajednice](#community-guidelines) i [kontekst](#context-options) se sastavljaju.
- LLM se poziva.
- Model bira pozive alata i dostavlja obrazloženja + ocene poverenja.
- Izvršavanje se beleži sa **Dry Run** značkom kako bi se jasno razlikovalo od produkcijskih izvršavanja.

### Šta se ne izvršava u Dry Run režimu

- Nijedan komentar se ne objavljuje, nijedan glas se ne daje, nijedan komentar se ne kači/skida sa kačenja/zaključava/otključava.
- Nijedan komentar se ne označava kao spam, ne odobrava i ne pregleda.
- Nijedan korisnik se ne banuje, ne upozorava i ne nagrađuje značkom.
- Nijedan email se ne šalje.
- Ništa se ne upisuje u memoriju. (Da - uključujući memoriju. Agenti u dry-run režimu ne mogu da zagade zajednički bazen memorije hipotetičkim odlukama.)
- Webhook-ovi se ne aktiviraju za radnje alata. (Webhook-ovi na nivou okidača `trigger.succeeded` / `trigger.failed` se i dalje aktiviraju i payload uključuje `wasDryRun: true`. Pogledajte [Webhook Payloads](#webhook-payloads).)

### Koliko košta

Dry Run izvršava **isti LLM poziv** koji bi izvršio i Enabled režim. Tokeni se naplaćuju, primenjuju se [budget cap ograničenja](#budgets-overview), a izvršavanja se računaju u dnevne/mesečne limite po agentu i po tenantu.

Ta cena je cena dobijanja vernog pregleda. Režim "preskoči LLM poziv" ne bi dao nikakav signal o tome kako bi se agent ponašao.

### Čitanje rezultata dry-run izvršavanja

U [Run History](#run-history), dry-run izvršavanja su označena **Dry Run** značkom u koloni statusa. Radnje unutar svakog izvršavanja izgledaju identično produkcijskim radnjama - isti naziv alata, isti argumenti, isto obrazloženje i poverenje - osim što se nijedna nije desila.

[Analytics stranica](#analytics-page) prikazuje raspodelu "dry-run vs live" izvršavanja po mesecu, tako da možete videti koliko je vaše potrošnje tokena otišlo na posmatranje.

### Prelazak iz Dry Run režima

Izmenite agenta i promenite **Status** na **Enabled**. Sledeći okidač se izvršava u produkcijskom režimu.

Možete se prebaciti i u drugom smeru - iz Enabled nazad u Dry Run - ako agent počne da radi stvari koje vam se ne sviđaju. Ne postoji nikakva kazna.

### Replays forsiraju Dry Run

Funkcija [Test Runs (Replays)](#test-runs-replays) pokreće agenta nad istorijskim komentarima **uvek u dry-run režimu**, bez obzira na sačuvani status agenta. Replays ne mogu da izvrše stvarne radnje nad prošlim komentarima. Ovo je po dizajnu - replay je alat za pregled, ne alat za moderaciju.