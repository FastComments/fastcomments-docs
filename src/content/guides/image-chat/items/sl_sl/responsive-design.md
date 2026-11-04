### Odstotkovno pozicioniranje

Image Chat uporablja odstotkovne koordinate namesto koordinat v piksilih za pozicioniranje označevalcev klepeta na slikah. Ko uporabnik klikne na sliko, pripomoček pretvori koordinate klika v piksilih v odstotke širine in višine slike. Ta pristop zagotavlja, da označevalci ostanejo na pravilnem mestu ne glede na to, kako je slika prikazana.

Na primer, če uporabnik klikne 250 pikslov od levega roba slike široke 1000px, pripomoček to shrani kot 25 % od leve. Ko drug uporabnik ista slika prikže široko 500px na mobilni napravi, se označevalec prikaže 125 pikslov od leve, kar je še vedno 25 % širine.

### Prednosti za odzivne postavitve

Ta odstotkovni sistem omogoča, da Image Chat brez težav deluje na vseh velikostih in usmerjenostih naprav. Vaše slike so morda prikazane v različnih velikostih, odvisno od širine zaslona, pravil CSS ali omejitev vsebnika, vendar se označevalci vedno pravilno poravnajo z želenimi lokacijami.

Uporabniki na namiznih računalnikih z velikimi zasloni vidijo označevalce na istih relativnih položajih kot uporabniki pametnih telefonov z majhnimi zasloni. Označevalci se razmeroma povečujejo s samo sliko.

### Povečevanje slik in razmerje stranic

Dokler vaša slika ob povečevanju ohranja razmerje stranic (kar je privzeto vedenje brskalnika), bodo odstotkovni označevalci ostali popolnoma poravnani. Pripomoček predpostavlja, da se slike povečujejo sorazmerno, in izračunuje položaje na podlagi te predpostavke.

Če uporabite CSS, ki popači razmerje stranic slike (na primer `object-fit: cover` z določenimi dimenzijami), se položaji označevalcev morda ne bodo pravilno poravnali. Za najboljše rezultate pustite, da se slike naravno povečujejo, ali uporabite `object-fit: contain` za ohranjanje razmerja stranic.

### Velikost klepetalnih kvadratkov

Vidna velikost označevalcev klepeta je prav tako odstotkovna. Možnost konfiguracije `chatSquarePercentage` ima privzeto vrednost 5 %, kar pomeni, da je vsak kvadratek 5 % širine slike. To zagotavlja dosledno vizualno težo pri različnih velikostih slik.

Na sliki široki 1000px s privzeto nastavitvijo 5 % so označevalci kvadratki 50px. Na sliki široki 500px so isti označevalci kvadratki 25px. Ostanejo sorazmerni z velikostjo slike.

### Vedenje na mobilnih napravah

Na zaslonih, ožjih od 768px, Image Chat preklopi na mobilno optimizirano postavitev. Klepetalna okna se odprejo v celozaslonskem načinu namesto da bi lebdeli ob označevalcu. To zagotavlja boljšo uporabnost na majhnih zaslonih, kjer bi ležeča okna zakrivala preveč slike.

Označevalci sami ostanejo vidni in klikljivi na svojih odstotkovnih položajih. Uporabniki lahko še vedno vidijo, kje se nahajajo vse razprave, in tapnejo označevalce, da odprejo celozaslonski klepetalni vmesnik.

### Dinamično nalaganje slik

Odstotkovni sistem deluje pravilno tudi, ko se slike nalagajo asinhrono ali spremenijo velikost po nalaganju strani. Pripomoček spremlja element slike in znova izračuna položaje označevalcev, kadar koli se dimenzije slike spremenijo.

Če lenobno nalagate slike ali izvajate odzivne slike z različnimi velikostmi pri različnih prelomnih točkah, se označevalci samodejno prilagodijo, ko se velikost slike spremeni.

### Doslednost med napravami

Ker so koordinate v podatkovni bazi shranjene kot odstotki, se razprava, ustvarjena na namiznem računalniku, prikaže na popolnoma istem relativnem mestu, ko si jo ogledujete na tablici ali telefonu. Uporabniki lahko sodelujejo med napravami brez kakršnih koli nedoslednosti pri pozicioniranju.

To deluje v obeh smereh. Razprava, ustvarjena s tapkanjem določene točke na mobilni napravi, se prikaže na istem relativnem položaju, ko si jo ogledujete na velikem namiznem zaslonu.

### Vidno območje

Pripomoček izračunava odstotke glede na sam element slike, ne glede na vidno območje. To pomeni, da drsenje strani ali spreminjanje velikosti okna brskalnika ne vpliva na položaje označevalcev. Označevalci ostanejo sidrani na svojih lokacijah na sliki ne glede na spremembe vidnega območja.

### Prihodnja odpornost vsebine

Odstotkovni pristop naredi vaše slikovne razprave odporne na spremembe postavitve, oblikovanja ali ekosistema naprav. Ko se pojavijo nove velikosti zaslonov in naprave, bodo obstoječe razprave še naprej pravilno prikazane brez potrebe po posodobitvah ali migracijah.