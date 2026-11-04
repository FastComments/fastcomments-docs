### Pozicioniranje na temelju postotaka

Image Chat koristi koordinate na temelju postotaka umjesto pikselnih koordinata za pozicioniranje oznaka razgovora na slikama. Kada korisnik klikne na sliku, widget pretvara pikselne koordinate klika u postotke širine i visine slike. Ovaj pristup osigurava da oznake ostaju na ispravnoj lokaciji bez obzira na to kako je slika prikazana.

Na primjer, ako korisnik klikne 250 piksela od lijevog ruba slike širine 1000px, widget to sprema kao 25% s lijeva. Kada drugi korisnik pregledava istu sliku širine 500px na mobilnom uređaju, oznaka se pojavljuje 125 piksela od lijevog ruba, što je i dalje 25% širine.

### Prednosti za responzivne rasporede

Ovaj postotni sustav omogućuje da Image Chat besprijekorno radi na svim veličinama i orijentacijama uređaja. Vaše slike mogu biti prikazane u različitim veličinama ovisno o širini zaslona, CSS pravilima ili ograničenjima spremnika, ali se oznake uvijek ispravno poravnavaju s namijenjenim lokacijama.

Korisnici na stolnim računalima s velikim monitorima vide oznake u istim relativnim pozicijama kao korisnici na pametnim telefonima s malim zaslonima. Oznake se skaliraju proporcionalno sa samom slikom.

### Skaliranje slika i omjer slike

Dok god vaša slika zadržava omjer pri skaliranju (što je zadano ponašanje preglednika), oznake na temelju postotaka ostat će savršeno poravnate. Widget pretpostavlja da se slike skaliraju proporcionalno i izračunava pozicije na temelju te pretpostavke.

Ako primijenite CSS koji iskrivljuje omjer slike (poput korištenja `object-fit: cover` s određenim dimenzijama), pozicije oznaka možda se neće ispravno poravnati. Za najbolje rezultate dopustite slikama da se prirodno skaliraju ili koristite `object-fit: contain` za očuvanje omjera slike.

### Veličina kvadrata razgovora

Vizualna veličina oznaka razgovora također se temelji na postocima. Konfiguracijska opcija `chatSquarePercentage` ima zadanu vrijednost od 5%, što znači da je svaki kvadrat 5% širine slike. To osigurava dosljednu vizualnu težinu u različitim veličinama slika.

Na slici širine 1000px sa zadanom postavkom od 5%, oznake su kvadrati od 50px. Na slici širine 500px, iste oznake su kvadrati od 25px. Ostaju proporcionalne veličini slike.

### Ponašanje na mobilnim uređajima

Na zaslonima širine manje od 768px, Image Chat prelazi na mobilno optimizirani raspored. Prozori razgovora otvaraju se preko cijelog zaslona umjesto da lebde uz oznaku. To pruža bolju upotrebljivost na malim zaslonima gdje bi leteći prozori prekrili previše slike.

Same oznake ostaju vidljive i klikabilne na svojim pozicijama temeljenim na postocima. Korisnici i dalje mogu vidjeti gdje se nalaze svi razgovori i dodirnuti oznake za otvaranje sučelja razgovora preko cijelog zaslona.

### Dinamičko učitavanje slika

Postotni sustav ispravno radi čak i kada se slike učitavaju asinkrono ili mijenjaju veličinu nakon učitavanja stranice. Widget prati element slike i ponovno izračunava pozicije oznaka svaki put kada se dimenzije slike promijene.

Ako koristite lazy-loading slika ili implementirate responzivne slike s različitim veličinama na različitim prijelomnim točkama, oznake se automatski prilagođavaju kada se veličina slike promijeni.

### Dosljednost među uređajima

Budući da se koordinate spremaju kao postoci u bazi podataka, razgovor stvoren na stolnom računalu pojavljuje se na potpuno istoj relativnoj lokaciji kada se pregledava na tabletu ili telefonu. Korisnici mogu surađivati na različitim uređajima bez ikakvih nekonzistentnosti u pozicioniranju.

Ovo radi u oba smjera. Razgovor stvoren dodirivanjem određene točke na mobilnom uređaju pojavljuje se na istoj relativnoj poziciji kada se pregledava na velikom monitoru stolnog računala.

### Razmatranja o vidljivom području

Widget izračunava postotke u odnosu na sam element slike, a ne na vidljivo područje (viewport). To znači da pomicanje stranice ili promjena veličine prozora preglednika ne utječe na pozicije oznaka. Oznake ostaju usidrene na svojim lokacijama na slici bez obzira na promjene vidljivog područja.

### Buduća otpornost sadržaja

Pristup temeljen na postocima čini vaše slikovne razgovore otpornima na promjene u rasporedu, dizajnu ili ekosustavu uređaja. Kako se pojavljuju nove veličine zaslona i uređaji, postojeći razgovori nastavit će se ispravno prikazivati bez potrebe za bilo kakvim ažuriranjima ili migracijama.