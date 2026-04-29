A **preizkusni zagon** (imenovan tudi **ponovitev**) zažene agenta na naboru zgodovinskih komentarjev **brez izvajanja dejanskih akcij**. To je najhitrejši način za predogled vedenja agenta, preden gre v živo.

Dosegljivo s strani seznama agentov preko gumba **Preizkusni zagon** v vrstici vsakega agenta.

### Kaj počne

Platforma:

1. Izbere vzorec zgodovinskih komentarjev, ki ustrezajo obsegu agenta, v oknu, ki ga izberete.
2. Za vsak komentar zažene agenta od začetka do konca, kot da bi bil komentar pravkar objavljen - enako kontekst, isti LLM klic, ista izbira orodij, enake utemeljitve in ocene zaupanja.
3. Zabeleži vsako izvedbo kot suhi zagon, označeno tako, da ostane združena s ponovitvijo, iz katere izvira, in izključena iz pogledov živega zagona.
4. **Primerja** agentovo odločitev s tem, kar se je v resnici zgodilo s komentarjem - ali je bil kasneje odobren, označen kot neželena pošta, izbrisan, blokiran s strani mehanizma za spam itd.

Rezultat je razlikovanje po komentarju: "Agent pri ponovitvi bi ta komentar označil kot spam, vendar je komentar trenutno odobren in čist."

### Konfiguracija

Stran za preizkusni zagon ima en vhod:

- **Dnevi zgodovinskih komentarjev za oceno** - številčno polje `days` med 1 in 90. Starejši komentarji niso upravičeni.

Velikost vzorca in trda omejitev nista **izpostavljeni v UI** - obe sta privzeti nastavitvi na strežniku, uporabljeni glede na načrt. Stran prikazuje informativna polja:

- **Ujemajoči se komentarji v oknu** - koliko komentarjev bi bilo upoštevanih.
- **Do N komentarjev iz tega okna bo obdelanih** - učinkovita velikost vzorca glede na strežniško omejitev.
- **Ocenjeni stroški** - v valuti vašega najemnika.

### Omejitev števila zahtev

Vsak uporabnik je omejen na **10 preizkusnih zagonov v 24 urah** (omejeno preko ključa `replay-create:${requestedBy}`). Gumb prikaže namig, ko dosežete limit ("Dosegli ste 10 preizkusnih zagonov v zadnjih 24 urah.").

### Sočasnost

Na agentu je lahko hkrati aktiven le en ponovitev. Začetek druge ponovitve, medtem ko je ena v teku, vas preusmeri na tisto, ki je v teku.

### Branje rezultatov

Ko ponovitev konča, stran z rezultati prikaže zavihke:

- **Diference** (privzeto aktiven) - agent pri ponovitvi se razlikuje od realnosti. (Najbolj zanimivo - "agent bi ta komentar označil kot spam, vendar je komentar odobren in v redu".)
- **Ujemanja** - agent pri ponovitvi se ujema s tem, kar se je dejansko zgodilo. (Pomirjujoče - agent se strinja z realnostjo.)
- **Brez akcije** - agent pri ponovitvi se je odločil, da ne naredi nič. (Včasih prava odločitev; včasih je agent kaj spregledal.)
- **Vse** - vsak rezultat ne glede na klasifikacijo.

Za vsak komentar v kateremkoli zavihku:

- **Prejšnji izid** - klasifikacija tega, kar se je dejansko zgodilo: **POZITIVNO**, **NEGATIVNO** ali **NEODLOČNO**, z **dokazi** ("Komentar označen kot izbrisan dne {date}", "Mehanizem: bayes" itd.).
- **Agent pri ponovitvi bi** - dejanje, ki ga je agent izbral.
- **Zakaj** - utemeljitev.
- **Zaupanje** - prikazano kot odstotek.

### Zakaj ponovitve prisilijo suhi zagon

Ponovitev proti komentarju, ki je bil izbrisan pred štirimi meseci, ga ne bi smela retroaktivno izbrisati - že je izbrisan. Ponovitev proti komentarju, ki ga agent zdaj želi odobriti, ne bi smela spremeniti trenutnega stanja komentarja. Ponovitev je orodje za predogled. Prisilitev suhega zagona je tisto, kar omogoča varno izvajanje ponovitve nad katerimkoli zgodovinskim oknom.

### Ponovljivost

Ponovitve zamrznejo konfiguracijo agenta v trenutku, ko je bila ponovitev začeta. Kasnejše spremembe agenta ne spremenijo rezultatov ponovitve - stran z rezultati ostane stabilna kot zapis tega, kaj bi *ta* različica agenta naredila.

### Ko proračuni ustavijo ponovitev

Ponovitve so podvržene:

- Lastni **strogi omejitvi** (nastavljeni na obrazcu za ponovitev).
- Dnevnim in mesečnim **omejitvam proračuna** agenta.
- Dnevnim in mesečnim **omejitvam proračuna** najemnika.

Prva dosežena omejitev prekine ponovitev s specifično kodo napake. Vsi rezultati po komentarjih, ustvarjeni pred prekinitvijo, so shranjeni v [Zgodovina zagonov](#run-history).

### Kako potekajo ponovitve

Ponovitve tečejo v ozadju, ne sinhrono. Ko kliknete "Start test run", je ponovitev postavljena v vrsto in delavec jo prevzame. Dolga ponovitev lahko traja več minut. Stran z rezultati periodično preverja in prikazuje napredek (število obdelanih, doslej porabljeno) med izvajanjem.

Če delavec odpove sredi ponovitve, platforma samodejno ponovno postavi ponovitev v vrsto, da se nadaljuje ob naslednjem poskusu. Kratkotrajen prekinitev ne pusti ponovitve brez nadaljevanja.

### Česa ponovitev ne stori

- **Ne upošteva [zamike sprožilcev](#trigger-deferred-delay).** Ponovitve tečejo takoj, ne čez 30 minut.
- **Ne zapisuje v pomnilnik.** Agenti pri ponovitvi ne shranjujejo zapiskov v pomnilnik, tudi če bi jih njihova logika sicer običajno ustvarila.
- **Ne sproži spletnih klicev (webhookov).** Sprožilci, ustvarjeni v ponovitvi, ne generirajo webhook dogodkov `trigger.succeeded` / `trigger.failed`.
- **Ne izključuje že ponovljenih komentarjev.** Zagon druge ponovitve nad istim oknom obravnava iste komentarje.

### Glej tudi

- [Izboljševanje pozivov](#refining-prompts) - delovni tok iterativnega urejanja, ki se dobro dopolnjuje s ponovitvami.
- [Način suhega zagona](#dry-run-mode) - ista ideja, na živo prometu.