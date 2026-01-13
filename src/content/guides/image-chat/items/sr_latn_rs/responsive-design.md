### Pozicioniranje zasnovano na procentima

Image Chat koristi koordinate zasnovane na procentima umesto piksel-koordinata za pozicioniranje markera chata na slikama. Kada korisnik klikne na sliku, widget konvertuje piksel-koordinate klika u procente širine i visine slike. Ovakav pristup osigurava da markeri ostanu na ispravnoj lokaciji bez obzira na to kako se slika prikazuje.

Na primer, ako korisnik klikne 250 piksela od levog ruba slike širine 1000px, widget to čuva kao 25% od leve strane. Kada drugi korisnik pogleda istu sliku širine 500px na mobilnom uređaju, marker se pojavljuje na 125 piksela od leve strane, što je i dalje 25% širine.

### Prednosti za responzivne rasporede

Ovaj sistem zasnovan na procentima omogućava da Image Chat radi besprekorno na svim veličinama uređaja i orijentacijama. Vaše slike mogu biti prikazane u različitim veličinama u zavisnosti od širine ekrana, CSS pravila ili ograničenja kontejnera, ali markeri su uvek pravilno poravnati sa predviđenim lokacijama.

Korisnici na desktop računarima sa velikim monitorima vide markere na istim relativnim pozicijama kao i korisnici na pametnim telefonima sa malim ekranima. Markeri se proporcionalno skaliraju zajedno sa samom slikom.

### Skaliranje slike i odnos stranica

Sve dok vaša slika održava odnos stranica pri skaliranju (što je podrazumevano ponašanje pretraživača), markeri zasnovani na procentima će ostati savršeno poravnati. Widget pretpostavlja da se slike skaliraju proporcionalno i izračunava pozicije na osnovu te pretpostavke.

Ako primenite CSS koji izobličava odnos stranica slike (npr. koristeći `object-fit: cover` sa specifičnim dimenzijama), pozicije markera možda neće biti pravilno poravnate. Za najbolje rezultate, dozvolite slikama da se prirodno skaliraju ili koristite `object-fit: contain` da održite odnos stranica.

### Veličina kvadrata markera

Vizuelna veličina chat markera je takođe zasnovana na procentima. Opcija konfiguracije `chatSquarePercentage` ima podrazumevanu vrednost 5%, što znači da je svaki kvadrat 5% širine slike. Ovo osigurava konzistentnu vizuelnu težinu na različitim veličinama slika.

Na slici širine 1000px sa podrazumevanih 5%, markeri su kvadratni od 50px. Na slici širine 500px, isti markeri su kvadratni od 25px. Oni ostaju proporcionalni veličini slike.

### Ponašanje na mobilnim uređajima

Na ekranima širinе ispod 768px, Image Chat prelazi na izgled optimizovan za mobilne uređaje. Chat prozori se otvaraju celozaslonski umesto da plutaju pored markera. Ovo pruža bolju upotrebljivost na malim ekranima gde bi plutajući prozori previše zaklanjali sliku.

Sami markeri ostaju vidljivi i klikabilni na svojim pozicijama zasnovanim na procentima. Korisnici i dalje mogu videti gde se nalaze sve diskusije i dodirnuti markere da otvore celozaslonski chat interfejs.

### Dinamičko učitavanje slika

Sistem zasnovan na procentima radi ispravno čak i kada se slike učitavaju asinhrono ili menjaju veličinu nakon učitavanja stranice. Widget nadgleda element slike i preračunava pozicije markera kad god se dimenzije slike promene.

Ako koristite leno učitavanje slika (lazy-loading) ili implementirate responzivne slike sa različitim veličinama na različitim breakpoint-ovima, markeri se automatski prilagođavaju kada se veličina slike promeni.

### Doslednost između uređaja

Pošto se koordinate čuvaju kao procenti u bazi podataka, diskusija kreirana na desktop računaru pojavljuje se na istoj relativnoj lokaciji kada se pregleda na tabletu ili telefonu. Korisnici mogu sarađivati preko uređaja bez bilo kakvih neslaganja u pozicioniranju.

Ovo radi obostrano. Diskusija kreirana dodirom specifičnog mesta na mobilnom uređaju pojavljuje se na istoj relativnoj poziciji kada se pregleda na velikom desktop monitoru.

### Razmatranja vezana za viewport

Widget izračunava procente u odnosu na sam element slike, a ne u odnosu na viewport. To znači da skrolovanje stranice ili promena veličine prozora pregledača ne utiču na pozicije markera. Markeri ostaju pričvršćeni za svoje lokacije na slici bez obzira na promene viewport-a.

### Priprema sadržaja za budućnost

Pristup zasnovan na procentima čini vaše diskusije na slikama otpornim na promene u rasporedu, dizajnu ili ekosistemu uređaja. Kako se pojavljuju nove veličine ekrana i uređaji, postojeće diskusije će i dalje biti prikazane ispravno bez potrebe za ažuriranjima ili migracijama.