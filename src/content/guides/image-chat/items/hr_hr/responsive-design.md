### Percentage-Based Positioning

Image Chat koristi koordinate temeljene na postocima umjesto pikselnih koordinata za pozicioniranje markera razgovora na slikama. Kada korisnik klikne na sliku, widget pretvara pikselne koordinate klika u postotke širine i visine slike. Ovaj pristup osigurava da markeri ostanu na ispravnom mjestu bez obzira na način prikaza slike.

Na primjer, ako korisnik klikne 250 piksela od lijeve ivice slike širine 1000px, widget to pohranjuje kao 25% od lijeve strane. Kada drugi korisnik pogleda istu sliku široku 500px na mobilnom uređaju, marker se pojavljuje 125 piksela od lijeve strane, što je opet 25% širine.

### Benefits for Responsive Layouts

Sustav temeljen na postocima omogućuje da Image Chat besprijekorno radi na svim veličinama i orijentacijama uređaja. Vaše se slike mogu prikazivati u različitim veličinama ovisno o širini ekrana, CSS pravilima ili ograničenjima kontejnera, ali se markeri uvijek pravilno poravnaju s namjenjenim lokacijama.

Korisnici na stolnim računalima s velikim monitorima vide markere na istim relativnim pozicijama kao i korisnici na pametnim telefonima s malim ekranima. Markeri se proporcionalno skaliraju sa samom slikom.

### Image Scaling and Aspect Ratio

Dokle god vaša slika zadržava svoj omjer širine i visine pri skaliranju (što je zadano ponašanje preglednika), markeri temeljeni na postocima ostat će savršeno poravnati. Widget pretpostavlja da se slike proporcionalno skaliraju i izračunava pozicije na temelju te pretpostavke.

Ako primijenite CSS koji izobličuje omjer slike (kao npr. korištenjem `object-fit: cover` s određenim dimenzijama), pozicije markera možda se neće pravilno poravnati. Za najbolje rezultate dopustite slikama da se prirodno skaliraju ili koristite `object-fit: contain` kako biste održali omjer.

### Chat Square Sizing

Vizualna veličina markera razgovora također je temeljena na postotcima. Opcija konfiguracije `chatSquarePercentage` zadano je postavljena na 5%, što znači da je svaki kvadrat 5% širine slike. To osigurava konzistentnu vizualnu težinu na različitim veličinama slika.

Na slici širine 1000px sa zadanim postavkama od 5%, markeri su kvadratni od 50px. Na slici širine 500px isti markeri su kvadratni od 25px. Oni ostaju proporcionalni veličini slike.

### Mobile Behavior

Na zaslonima širine manjom od 768px, Image Chat prelazi na izgled optimiziran za mobilne uređaje. Prozori razgovora se otvaraju preko cijelog zaslona umjesto da lebde pored markera. To poboljšava upotrebljivost na malim ekranima gdje bi lebdeći prozori previše zakrivali sliku.

Sami markeri ostaju vidljivi i klikabilni na svojim pozicijama temeljenim na postocima. Korisnici i dalje mogu vidjeti gdje se nalaze sve rasprave i dodirnuti markere kako bi otvorili sučelje za razgovor preko cijelog zaslona.

### Dynamic Image Loading

Sustav temeljen na postocima radi ispravno čak i kada se slike učitavaju asinkrono ili mijenjaju veličinu nakon učitavanja stranice. Widget prati element slike i preračunava pozicije markera kad god se dimenzije slike promijene.

Ako koristite lijeno učitavanje slika ili implementirate responzivne slike s različitim veličinama na različitim prekidnim točkama, markeri se automatski prilagođavaju kada se veličina slike promijeni.

### Cross-Device Consistency

Budući da se koordinate pohranjuju u bazi podataka kao postoci, rasprava stvorena na stolnom računalu pojavljuje se na točno istoj relativnoj lokaciji kada se pogleda na tabletu ili telefonu. Korisnici mogu surađivati preko uređaja bez bilo kakvih neslaganja u pozicioniranju.

Ovo radi obostrano. Rasprava stvorena dodirom određene točke na mobilnom uređaju pojavljuje se na istoj relativnoj poziciji kada se pogleda na velikom stolnom monitoru.

### Viewport Considerations

Widget izračunava postotke u odnosu na sam element slike, a ne u odnosu na viewport. To znači da pomicanje stranice ili promjena veličine prozora preglednika ne utječe na pozicije markera. Markeri ostaju vezani za svoje lokacije na slici bez obzira na promjene prikaza.

### Future-Proofing Content

Pristup temeljen na postocima čini vaše rasprave o slikama otpornima na promjene u izgledu, dizajnu ili ekosustavu uređaja. Kako se pojavljuju nove veličine zaslona i uređaji, postojeće rasprave nastavit će se pravilno prikazivati bez potrebe za ažuriranjima ili migracijama.