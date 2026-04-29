Privzeto agent teče **takoj** po sprožitvi njegovega sprožilca. Polje **Zamik pred izvajanjem** na obrazcu za urejanje to spremeni: platforma uvrsti sprožilec v čakalno vrsto in izvede agenta ob načrtovanem času.

### Kdaj uporabiti zamik

- **Sprožilci po pragu prijav (flag-threshold triggers)** - prijave se pogosto pojavijo v sunkih. 10–30-minutni zamik pusti, da se slika umiri, tako da agent deluje na končnem številu prijav in ne na številu ob trenutnem prihodu.
- **Sprožilci po pragu glasov (vote-threshold triggers)** - ista logika, predvsem pri brigadiranju z negativnimi glasovi.
- **Povzemanje niti** - [Predloga za povzetek niti](#template-thread-summarizer) privzeto nastavi 30-minutni zamik, tako da povzame pogovor, ki se je imel čas razviti, ne pa niti z dvema odgovoroma.
- **Ohladitev / ponovna ocena** - "24 ur po tem, ko je komentar zaklenjen, premislite, ali ga odkleniti."

### Konfiguracija

- **Polje**: Zamik pred izvajanjem.
- **Obseg**: 0 do 2,592,000 sekund (30 dni).
- **Enote**: Sekunde, minute, ure ali dnevi.

### Idempotentnost

Odložena čakalna vrsta ne odstrani podvojenih sprožilcev. Dve prijavi, ki prispeta z eno sekundo razmika pri agentu z 30-minutnim zamikom, bosta oba razporejeni v zagon čez 30 minut, agent pa bo tekel **dvakrat**, obakrat z (večinoma) istim kontekstom. Če želite zagotoviti največ en zagon na okno, mora to uvesti agent sam — običajno tako, da ob prvem izvajanju zapiše [spominsko opombo](#tools-overview) in jo preveri pri naslednjih zagonih.

### Opomba o stroških

Odloženi sprožilci so zabeleženi **pred** izvajanjem. Sunek sprožilk pri agentu z velikim zamikom se lahko nakopiči v čakalni vrsti, ne da bi porabil žetone; stroške poravnate šele, ko jih cron dejansko izvede. Uporabite [Zgodovino zaganjanj](#run-history) in [Razloge za zavrženje](#drop-reasons), da vidite, kako pogosto se odloženi sprožilci dejansko izvedejo v primerjavi s tem, koliko jih je pri izvajanju opuščenih zaradi proračunskih omejitev.

### Predvajanje ne upošteva zamika

Funkcija [Testni zagoni (ponovitve)](#test-runs-replays) izvede agenta takoj nad zgodovinskimi komentarji - ne čaka na konfiguriran zamik. Obnašajte se, kot da gre za funkcijo: predvajanja služijo za predogled tega, kaj bi agent **storil** glede na kontekst, ne pa za reprodukcijo razporeditve v realnem času.