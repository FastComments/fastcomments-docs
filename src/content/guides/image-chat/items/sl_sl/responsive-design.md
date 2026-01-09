### Percentage-Based Positioning

Image Chat uporablja koordinate, izražene v odstotkih, namesto pikselskih koordinat za pozicioniranje oznak klepeta na slikah. Ko uporabnik klikne na sliko, pripomoček pretvori pikselske koordinate klika v odstotke širine in višine slike. Ta pristop zagotavlja, da oznake ostanejo na pravem mestu ne glede na to, kako je slika prikazana.

Na primer, če uporabnik klikne 250 slikovnih pik od levega roba slike širine 1000 px, pripomoček to shrani kot 25% od leve strani. Ko drug uporabnik pogleda isto sliko na mobilni napravi, kjer je široka 500 px, se oznaka prikaže 125 slikovnih pik od leve, kar je še vedno 25% širine.

### Benefits for Responsive Layouts

Ta sistem z odstotki omogoča, da Image Chat nemoteno deluje na vseh velikostih in orientacijah naprav. Vaše slike so lahko prikazane v različnih velikostih glede na širino zaslona, CSS pravila ali omejitve vsebnika, vendar se oznake vedno pravilno poravnajo s predvidenimi lokacijami.

Uporabniki na namiznih računalnikih z velikimi monitorji vidijo oznake na enakih sorazmernih položajih kot uporabniki na pametnih telefonih z majhnimi zasloni. Oznake se skalirajo sorazmerno s samo sliko.

### Image Scaling and Aspect Ratio

Dokler vaša slika pri spreminjanju velikosti ohranja svoj razmerje stranic (kar je privzeto vedenje brskalnika), bodo oznake, izražene v odstotkih, ostale popolnoma poravnane. Pripomoček predvideva, da se slike skalirajo sorazmerno in izračunava položaje na podlagi tega predpostavka.

Če uporabite CSS, ki izkrivlja razmerje stranic slike (na primer z uporabo `object-fit: cover` s specifičnimi dimenzijami), se položaji oznak morda ne bodo pravilno poravnali. Za najboljše rezultate dovolite slikam, da se naravno skalirajo, ali uporabite `object-fit: contain`, da ohranite razmerje stranic.

### Chat Square Sizing

Vidna velikost oznak klepeta je prav tako izražena v odstotkih. Konfiguracijska možnost `chatSquarePercentage` privzeto znaša 5%, kar pomeni, da je vsako polje veliko 5% širine slike. To zagotavlja enako vizualno težo na različnih velikostih slik.

Na sliki širine 1000 px z privzeto nastavitvijo 5% so oznake kvadratne s 50 px. Na sliki širine 500 px so iste oznake kvadratne s 25 px. Ostanejo sorazmerne velikosti slike.

### Mobile Behavior

Na zaslonih, ožjih od 768 px, se Image Chat preklopi na postavitev, optimizirano za mobilne naprave. Okna klepeta se odprejo celozaslonsko namesto plavajoče ob oznaki. To izboljša uporabnost na majhnih zaslonih, kjer bi plavajoča okna preveč zakrivala sliko.

Sami oznake ostanejo vidne in klikabilne na svojih položajih, izraženih v odstotkih. Uporabniki lahko še vedno vidijo, kje so vse razprave, in tapnejo oznake, da odprejo celozaslonsni vmesnik klepeta.

### Dynamic Image Loading

Sistem, ki temelji na odstotkih, deluje pravilno tudi, ko se slike nalagajo asinhrono ali spremenijo velikost po nalaganju strani. Pripomoček spremlja element slike in ponovno izračuna položaje oznak vsakič, ko se spremenijo dimenzije slike.

Če uporabljate odloženo nalaganje slik ali uvajate odzivne slike z različnimi velikostmi na različnih prelomih, se oznake samodejno prilagodijo, ko se velikost slike spremeni.

### Cross-Device Consistency

Ker so koordinate shranjene v bazi kot odstotki, se razprava, ustvarjena na namiznem računalniku, prikaže na enakem sorazmernem mestu tudi na tablici ali telefonu. Uporabniki lahko sodelujejo med napravami brez neskladij v pozicioniranju.

To deluje v obe smeri. Razprava, ustvarjena z dotikom določene točke na mobilni napravi, se prikaže na enakem sorazmernem položaju, ko jo pogledate na velikem namiznem monitorju.

### Viewport Considerations

Pripomoček izračunava odstotke glede na sam element slike, ne na pogledno okno (viewport). To pomeni, da pomikanje po strani ali spreminjanje velikosti okna brskalnika ne vpliva na položaje oznak. Oznake ostanejo pritrjene na svojih lokacijah na sliki ne glede na spremembe viewporta.

### Future-Proofing Content

Pristop, ki temelji na odstotkih, naredi vaše razprave o slikah odporne na spremembe postavitve, dizajna ali ekosistema naprav. Ko se pojavijo nove velikosti zaslonov in naprave, se obstoječe razprave še naprej pravilno prikazujejo brez potrebe po posodobitvah ali migracijah.