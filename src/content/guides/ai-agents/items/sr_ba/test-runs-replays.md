Jedno **test pokretanje** (takođe nazvano **reprodukcija**) pokreće agenta nad prozorom istorijskih komentara **bez preduzimanja stvarnih radnji**. To je najbrži način da pregledate ponašanje agenta prije puštanja u rad.

Dostupno sa stranice liste agenata putem dugmeta **Test run** u svakom redu agenta.

### Šta radi

Platforma:

1. Odabire uzorak istorijskih komentara koji odgovaraju obimu agenta, u prozoru koji odaberete.
2. Za svaki komentar pokreće agenta end-to-end kao da je komentar upravo objavljen - isti kontekst, isti LLM poziv, isti izbor alata, ista opravdanja i iste ocjene povjerenja.
3. Snima svako pokretanje kao dry-run, označavajući ga tako da ostane grupisano sa reprodukcijom od koje potiče i isključeno iz prikaza live-run.
4. **Upoređuje** presudu agenta sa onim što se zapravo desilo komentaru - da li je kasnije odobren, označen kao spam, obrisan, blokiran od strane spam motora, itd.

Rezultat je diff po komentaru: "Agent u reprodukciji bi označio ovo kao spam, ali komentar je trenutno odobren i čist."

### Konfiguracija

Stranica test-pokretanja ima jedan unos:

- **Days of historical comments to evaluate** - numeričko polje `days` između 1 i 90. Stariji komentari nisu podobni.

Veličina uzorka i hard cap nisu izloženi u UI-ju - oba su server-side podrazumevana koja se primjenjuju po planu. Stranica prikazuje informativna polja:

- **Matching comments in window** - koliko komentara bi bilo razmotreno.
- **Up to N comments from this window will be processed** - efektivna veličina uzorka s obzirom na server-side cap.
- **Estimated cost** - u valuti vašeg tenanta.

### Ograničenje učestalosti

Svaki korisnik je ograničen na **10 test pokretanja u 24 sata** (rate-limited via key `replay-create:${requestedBy}`). Dugme prikazuje tooltip kada dostignete limit ("Dostigli ste 10 test pokretanja u posljednjih 24 sata.").

### Istovremenost

Samo jedna reprodukcija može biti aktivna po agentu u isto vrijeme. Pokretanje druge reprodukcije dok je jedna u toku preusmjerava vas na onu koja je u toku.

### Čitanje rezultata

Kada se reprodukcija završi, stranica rezultata prikazuje tabove:

- **Deltas** (podrazumevano aktivno) - presuda agenta u reprodukciji se razlikuje od stvarnosti. (Najinteresantnije - "agent bi označio ovaj komentar kao spam, ali komentar je odobren i u redu".)
- **Matches** - presuda agenta u reprodukciji se poklapa sa onim što se stvarno desilo. (Umirujuće - agent se slaže sa stvarnošću.)
- **No action** - agent u reprodukciji je odlučio da ne preduzme ništa. (Ponekad je to ispravan odgovor; ponekad je agent nešto propustio.)
- **All** - svaki rezultat bez obzira na klasifikaciju.

Za svaki komentar u bilo kojem tabu:

- **Prior outcome** - klasifikacija onoga što se zapravo desilo: **POSITIVE**, **NEGATIVE**, or **INDETERMINATE**, sa **Evidenca** ("Comment marked deleted at {date}", "Engine: bayes", i tako dalje).
- **Replay agent would** - akcija koju je agent odabrao.
- **Why** - opravdanje.
- **Confidence** - prikazano kao procenat.

### Zašto reprodukcije moraju biti dry-run

Reprodukcija nad komentarom koji je izbrisan prije četiri mjeseca ne bi trebala retroaktivno da ga izbriše - on je već izbrisan. Reprodukcija nad komentarom koji agent sada želi da odobri ne bi trebala promijeniti trenutni status komentara. Reprodukcija je alat za pregled. Zatezanje dry-run režima je ono što čini sigurno pokretanje reprodukcije nad bilo kojim istorijskim prozorom.

### Reproducibilnost

Reprodukcije zamrzavaju konfiguraciju agenta u trenutku kada je reprodukcija pokrenuta. Naknadne izmjene agenta ne mijenjaju rezultate reprodukcije - stranica rezultata ostaje stabilna kao zapis o tome šta bi ta verzija agenta uradila.

### Kada budžeti zaustave reprodukciju

Reprodukcije podležu:

- Svojoj sopstvenoj **hard cap** (postavljenoj na formi reprodukcije).
- Dnevnim i mjesečnim **budžetskim capovima** agenta.
- Dnevnim i mjesečnim **budžetskim capovima** tenanta.

Prvi koji se dostigne prekida reprodukciju sa specifičnim error kodom. Bilo koji po-komentarski rezultati proizvedeni prije aborta su sačuvani u [Run History](#run-history).

### Kako se reprodukcije izvršavaju

Reprodukcije se izvršavaju u pozadini, asinhrono. Nakon što kliknete "Start test run", reprodukcija se stavlja u red i radnik je preuzima. Duga reprodukcija može trajati nekoliko minuta. Stranica rezultata anketira i prikazuje napredak (broj obrađenih, potrošeno do sada) kako napreduje.

Ako radnik umre usred reprodukcije, platforma automatski vraća reprodukciju u red tako da se nastavi pri sljedećem prolazu. Kratki prekid nikada ne ostavlja reprodukciju bez nadzora.

### Šta reprodukcija ne radi

- **Ne poštuje [trigger delays](#trigger-deferred-delay).** Reprodukcije se izvršavaju odmah, ne poslije 30 minuta.
- **Ne zapisuje u memoriju.** Agenti u reprodukciji ne spremaju bilješke u memoriju, čak i ako bi njihova logika inače to radila.
- **Ne pokreće webhooks.** Trigger-i proizvedeni u reprodukciji ne generišu `trigger.succeeded` / `trigger.failed` webhook događaje.
- **Ne isključuje već reprodukovane komentare.** Pokretanje druge reprodukcije nad istim prozorom pokriva iste komentare.

### Pogledajte također

- [Usavršavanje upita](#refining-prompts) - workflow iterativnih izmjena koji dobro ide uz reprodukcije.
- [Dry-Run Mode](#dry-run-mode) - ista ideja, nad živim saobraćajem.