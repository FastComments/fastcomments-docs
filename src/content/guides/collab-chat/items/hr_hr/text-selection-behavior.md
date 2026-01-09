### Kako funkcionira odabir teksta

Kad korisnici odaberu tekst unutar Collab Chat kontejnera, widget bilježi taj odabir i omogućuje im pokretanje rasprave. Odabir može biti mali poput jedne riječi ili velik koliko više odlomaka koji obuhvaćaju različite elemente.

### Kašnjenje odabira

Na stolnim uređajima postoji kašnjenje od 3,5 sekunde između trenutka kada korisnik odabere tekst i pojave upita za raspravu. To sprječava treperenje korisničkog sučelja kada korisnici jednostavno označavaju tekst radi kopiranja ili čitanja. Na mobilnim uređajima upit se pojavljuje odmah jer je odabir teksta na zaslonima osjetljivim na dodir obično namjerniji.

### Jedinstveni ID-ovi razgovora

Svaki razgovor dobiva jedinstveni `urlId` koji kombinira URL stranice, putanju DOM elementa i serijalizirani raspon teksta. To osigurava da svaki odabir teksta stvori poseban razgovor koji se kasnije može ponovno pronaći.

Ako u svojoj konfiguraciji navedete prilagođeni `urlId`, on će se kombinirati s putanjom elementa i rasponom teksta kako bi se stvorio konačni identifikator.

### Vizualna isticanja

Kad za određeni odabir teksta postoji rasprava, taj tekst dobiva vizualno isticanje. Isticanje se ostvaruje pomoću boja pozadine i pojavljuje se pri prelasku mišem ili kada je pridruženi chat prozor otvoren.

Sustav isticanja radi tako da omota odabrani tekst u poseban element koji se može stilizirati. Ovaj pristup osigurava da isticanja ostanu točna čak i kada je temeljna HTML struktura složena.

### Pozicioniranje prozora chata

Kada korisnik klikne na isticanje ili stvori novu bilješku, pored odabranog teksta pojavi se prozor chata. Widget automatski izračunava najbolju poziciju za taj prozor na temelju dostupnog prostora unutar prikaza (viewport).

Sustav pozicioniranja koristi CSS klase poput `to-right`, `to-left`, `to-top` i `to-bottom` za označavanje u kojem smjeru bi se prozor chata trebao proširiti od isticanja. Na mobilnim uređajima (zasloni širi manje od 768px), prozor chata uvijek se pojavljuje preko cijelog zaslona radi bolje upotrebljivosti.

### Dimenzije chat prozora

Chat prozori na stolnim računalima široki su 410px s razmakom od 20px i vizualnom strelicom od 16px koja pokazuje na istaknuti tekst. Te dimenzije su fiksne kako bi se osiguralo dosljedno ponašanje, ali izgled možete prilagoditi pomoću CSS-a.

### Odabiri koji obuhvaćaju više elemenata

Korisnici mogu odabrati tekst koji se proteže kroz više HTML elemenata, primjerice označavanje od sredine jednog odlomka do početka drugog. Sustav serijalizacije raspona ispravno obrađuje ovo i istaknut će sav odabrani tekst čak i preko granica elemenata.

### Kompatibilnost preglednika

Sustav odabira teksta koristi standardni API `window.getSelection()` koji je podržan u svim modernim preglednicima. Za starije verzije Internet Explorera koristi se povratna kompatibilnost s `document.selection`.

### Postojanost odabira

Kad se za odabir teksta stvori razgovor, ta bilješka ostaje čak i ako se stranica ponovno učita. Serijalizirani raspon i DOM putanja omogućuju widgetu da ponovno obnovi isticanja na točno istom mjestu kada se korisnici vrate na stranicu.

Ovo radi pouzdano sve dok sadržaj vaše stranice ostane stabilan. Ako promijenite tekstualni sadržaj ili restrukturirate HTML, postojeće bilješke možda više neće pravilno odgovarati tekstu. Iz tog razloga najbolje je izbjegavati velike promjene sadržaja na stranicama s aktivnim bilješkama ili razmotriti migraciju bilješki kada su promjene sadržaja neophodne.