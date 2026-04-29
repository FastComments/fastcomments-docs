A **test run** (također nazvan **replay**) pokreće agenta protiv vremenskog prozora povijesnih komentara **bez poduzimanja stvarnih radnji**. To je najbrži način za pregled ponašanja agenta prije nego što prijeđete u produkciju.

Dostupno je s stranice popisa agenata putem gumba **Test run** u retku svakog agenta.

### Što radi

Platforma:

1. Odabire uzorak povijesnih komentara koji odgovaraju opsegu agenta, unutar prozora koji odaberete.
2. Za svaki komentar pokreće agenta od početka do kraja kao da je komentar upravo objavljen - isti kontekst, isti LLM poziv, isti odabir alata, iste opravdane odluke i ocjene pouzdanosti.
3. Zapisuje svako pokretanje kao dry-run, označeno tako da ostane grupirano s replayom iz kojeg potječe i isključeno iz prikaza live-runova.
4. **Uspoređuje** presudu agenta s onim što se zapravo dogodilo s komentarom - je li kasnije odobren, označen kao spam, izbrisan, blokiran od strane spam motora itd.

Rezultat je razlika po komentaru: "Replay bi ovaj komentar označio kao spam, ali komentar je trenutno odobren i čist."

### Konfiguracija

Stranica test runa ima jedno polje:

- **Days of historical comments to evaluate** - numeričko polje `days` između 1 i 90. Stariji komentari nisu podobni.

Veličina uzorka i gornja granica nisu **izložene u UI** - obje su zadane postavke na poslužitelju primijenjene po planu. Stranica prikazuje informativna polja:

- **Matching comments in window** - koliko komentara bi se uzelo u razmatranje.
- **Up to N comments from this window will be processed** - efektivna veličina uzorka s obzirom na gornju granicu na strani poslužitelja.
- **Estimated cost** - u valuti vašeg tenanta.

### Ograničenje brzine

Svaki korisnik ima ograničenje od **10 test runova u 24 sata** (rate-limited preko ključa `replay-create:${requestedBy}`). Gumb prikazuje tooltip kad dosegnете ograničenje ("Dosegli ste 10 test runova u posljednja 24 sata.").

### Istovremenost

Samo jedan replay može biti aktivan po agentu u isto vrijeme. Pokretanje drugog replaya dok je jedan u tijeku preusmjerava vas na onaj koji je u tijeku.

### Čitanje rezultata

Kada replay završi, stranica rezultata prikazuje kartice:

- **Deltas** (zadano aktivna) - replay agenta se razlikuje od stvarnosti. (Najzanimljivije - "agent bi ovaj komentar označio kao spam, ali komentar je odobren i u redu".)
- **Matches** - replay agenta se slaže s onim što se zapravo dogodilo. (Utešno - agent se slaže sa stvarnošću.)
- **No action** - replay agent je odlučio ništa ne poduzeti. (Ponekad ispravan odgovor; ponekad je agent nešto propustio.)
- **All** - svi rezultati bez obzira na klasifikaciju.

Za svaki komentar u bilo kojoj kartici:

- **Prior outcome** - klasifikacija onoga što se zapravo dogodilo: **POSITIVE**, **NEGATIVE**, ili **INDETERMINATE**, s **Evidence** ("Komentar označen kao izbrisan na {date}", "Engine: bayes", i slično).
- **Replay agent would** - akcija koju je agent odabrao.
- **Why** - opravdanje.
- **Confidence** - prikazano kao postotak.

### Zašto replayi prisiljavaju dry-run

Replay nad komentarom koji je izbrisan prije četiri mjeseca ne bi trebao retroaktivno izbrisati taj komentar - on je već izbrisan. Replay nad komentarom koji agent sada želi odobriti ne bi trebao promijeniti trenutni status komentara. Replay je alat za pregled. Prisila na dry-run je ono što ga čini sigurnim za pokretanje nad bilo kojim prozorom povijesti.

### Reproducibilnost

Replayi zamrzavaju konfiguraciju agenta u trenutku kada je replay pokrenut. Naknadne izmjene agenta ne mijenjaju rezultate replaya - stranica rezultata ostaje stabilna kao zapis što bi ta verzija agenta učinila.

### Kad budžeti zaustave replay

Replayi podliježu:

- Svojoj vlastitoj **gornjoj granici** (postavljenoj na obrascu replaya).
- Dnevnim i mjesečnim **ograničenjima budžeta** agenta.
- Dnevnim i mjesečnim **ograničenjima budžeta** tenanta.

Prvo pogodeno ograničenje prekida replay s određenim kodom pogreške. Bilo koji rezultati po komentaru proizvedeni prije prekida sačuvani su u [Run History](#run-history).

### Kako se replayi izvršavaju

Replayi se izvode u pozadini, ne sinkrono. Nakon što kliknete "Start test run", replay se stavi u red i radnik ga preuzima. Dugo trajanje replaya može potrajati nekoliko minuta. Stranica rezultata povlači podatke i pokazuje napredak (broj obrađenih, potrošnja do sada) kako napreduje.

Ako radnik umre usred replaya, platforma automatski vraća replay u red tako da nastavi pri sljedećem pokušaju. Kratkotrajni prekid nikada ne ostavlja replay napuštenim.

### Što replay ne radi

- **Ne poštuje [trigger delays](#trigger-deferred-delay).** Replays se izvršavaju odmah, a ne 30 minuta kasnije.
- **Ne zapisuje u memoriju.** Replay agenti ne spremaju bilješke u memoriju, čak i ako bi njihova logika to inače učinila.
- **Ne pokreće webhooks.** Okidači proizvedeni replayem ne generiraju `trigger.succeeded` / `trigger.failed` webhook događaje.
- **Ne isključuje već replayane komentare.** Pokretanje drugog replaya nad istim prozorom obuhvaća iste komentare.

### Vidi također

- [Refining Prompts](#refining-prompts) - workflow iterativnog uređivanja koji dobro ide uz replaye.
- [Dry-Run Mode](#dry-run-mode) - ista ideja, primijenjena na live promet.