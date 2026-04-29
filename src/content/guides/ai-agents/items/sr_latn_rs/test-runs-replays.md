A **test pokretanje** (takođe nazvano **replay**) pokreće agenta preko prozora istorijskih komentara **bez preduzimanja stvarnih akcija**. To je najbrži način da pregledate ponašanje agenta pre puštanja uživo.

Dostupno sa stranice liste agenata preko dugmeta **Pokreni test** u svakom redu agenta.

### Šta radi

Platforma:

1. Izabere uzorak istorijskih komentara koji odgovaraju opsegu agenta, u prozoru koji izaberete.
2. Za svaki komentar pokreće agenta end-to-end kao da je komentar upravo objavljen - isti kontekst, isti LLM poziv, isti izbor alata, iste opravdanja i skorove poverenja.
3. Beleži svako izvođenje kao dry-run, tagovano tako da ostane grupisano sa replay-jem iz kojeg potiče i isključeno iz pregleda uživo.
4. **Upoređuje** presudu agenta sa onim što se zapravo desilo sa komentarom - da li je kasnije odobren, označen kao spam, obrisan, blokiran od strane spam engine-a, itd.

Rezultat je diff po komentarima: "Replay agent bi ovo označio kao spam, ali komentar je trenutno odobren i čist."

### Konfiguracija

Stranica test-pokretanja ima jedan unos:

- **Days of historical comments to evaluate** - numeričko polje `days` između 1 i 90. Stariji komentari nisu podobni.

Veličina uzorka i hard cap **nisu izloženi u UI** - oba su podrazumevane vrednosti na serverskoj strani primenjene po planu. Stranica prikazuje informativna polja:

- **Matching comments in window** - koliko bi komentara bilo razmatrano.
- **Up to N comments from this window will be processed** - efektivna veličina uzorka uzimajući u obzir serverski hard cap.
- **Estimated cost** - u valuti vašeg tenant-a.

### Ograničenje brzine

Svaki korisnik je ograničen na **10 test pokretanja u toku 24 sata** (rate-limited putem ključa `replay-create:${requestedBy}`). Dugme prikazuje tooltip kada dostignete limit ("You've reached 10 test runs in the last 24 hours.").

### Paralelizam

Samo jedan replay može biti aktivan po agentu u datom trenutku. Pokretanje drugog replay-ja dok je jedan u toku preusmerava vas na onaj koji je trenutno u toku.

### Čitanje rezultata

Kada se replay završi, stranica rezultata prikazuje kartice:

- **Deltas** (podrazumevano aktivna) - presuda replay agenta razlikuje se od realnosti. (Najzanimljivije - "agent bi ovaj komentar označio kao spam, ali komentar je odobren i u redu".)
- **Matches** - presuda replay agenta se poklapa sa onim što se zapravo desilo. (Utešno - agent se slaže sa realnošću.)
- **No action** - replay agent je odlučio da ne preduzme ništa. (Ponekad je to pravi odgovor; ponekad je agent nešto propustio.)
- **All** - svaki rezultat bez obzira na klasifikaciju.

Za svaki komentar u bilo kojoj kartici:

- **Prior outcome** - klasifikacija onoga što se zapravo desilo: **POSITIVE**, **NEGATIVE**, ili **INDETERMINATE**, sa **Evidence** ("Comment marked deleted at {date}", "Engine: bayes", i slično).
- **Replay agent would** - akcija koju je agent izabrao.
- **Why** - opravdanje.
- **Confidence** - prikazano kao procenat.

### Zašto replay-ji forsiraju dry-run

Replay nad komentarom koji je obrisan pre četiri meseca ne bi trebalo retroaktivno da ga obriše - on je već obrisan. Replay nad komentarom koji agent sada želi da odobri ne bi trebao da promeni trenutni status komentara. Replay je alat za pregled. Forsiranje dry-run režima je to što ga čini bezbednim za pokretanje replay-ja protiv bilo kog istorijskog prozora.

### Reproducibilnost

Replay-ji zamrzavaju konfiguraciju agenta u trenutku kada je replay pokrenut. Kasniji izmeni agenta ne menjaju rezultate replay-ja - stranica rezultata ostaje stabilna kao zapis šta bi *ta* verzija agenta uradila.

### Kada budžeti zaustave replay

Replay-ji podležu:

- Sopstvenom **hard cap**-u (podešenom na formi za replay).
- Dnevnim i mesečnim **budget cap**-ovima agenta.
- Dnevnim i mesečnim **budget cap**-ovima tenant-a.

Prvi koji je dostignut prekida replay sa specifičnim kodom greške. Bilo koji rezultati po komentarima proizvedeni pre prekida su sačuvani u [Run History](#run-history).

### Kako replay-ji rade

Replay-ji se izvršavaju u pozadini, ne sinhrono. Nakon što kliknete "Start test run", replay se stavlja u red i jedan worker ga preuzima. Dug replay može trajati nekoliko minuta. Stranica rezultata periodično proverava i pokazuje napredak (broj obrađenih, potrošnja do sada) kako ide.

Ako worker crkne usred replay-ja, platforma automatski ponovo stavi replay u red tako da se nastavi pri sledećem prolazu. Kratki prekid nikada ne ostavlja replay siročetom.

### Šta replay ne radi

- **Ne poštuje [trigger delays](#trigger-deferred-delay).** Replay-ji se pokreću odmah, ne 30 minuta kasnije.
- **Ne zapisuje u memoriju.** Replay agenti ne čuvaju beleške u memoriji, čak i ako bi njihova logika to normalno radila.
- **Ne aktivira webhooks.** Trigger-i proizvedeni u replay-ju ne generišu `trigger.succeeded` / `trigger.failed` webhook događaje.
- **Ne isključuje već-replay-ovane komentare.** Pokretanje drugog replay-ja protiv istog prozora pokriva iste komentare.

### Vidi takođe

- [Refining Prompts](#refining-prompts) - workflow iterativnih izmena koji dobro ide uz replay-je.
- [Dry-Run Mode](#dry-run-mode) - ista ideja, primenjena na live saobraćaj.