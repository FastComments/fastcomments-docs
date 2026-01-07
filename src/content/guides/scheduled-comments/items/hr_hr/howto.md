Koristenje FastComments Zakazanih komentara je jednostavno. Prvo, zeljet cete [uvesti komentare ovdje](https://fastcomments.com/auth/my-account/manage-data/import-scheduled).

Ovoj stranici se takoder moze pristupiti putem `Manage Data->Schedule Comments`.

### Rucno reproducirani komentari

*Za rucno* reproducirane komentare (morate rucno pritisnuti Play), imate opciju poceti reproducirati komentare. Ovo ce reproducirati komentare na svakoj stranici koju ste definirali u CSV
datoteci, s kasnjenjem izmedu svakog komentara temeljenim na kasnjenju koje ste naveli.

Ovo je korisno kada imate zakazani webinar uzivo koji pocinje u odredeno vrijeme. Kada webinar pocne, samo pritisnite Play
na nadzornoj ploci.

### Automatska reprodukcija komentara

*Za automatski* reproducirane komentare, komentari se reproduciraju pri svakom ucitavanju stranice za svakog korisnika.

Ovo je korisno za scenarije gdje video ili drugi sadrzaj pocinje od pocetka sa svakim ucitavanjem.

### Dinamicki rastuca reprodukcija

Svaki put kada se skripta za automatsku reprodukciju pokrene za korisnika - pri ucitavanju stranice - jos uvijek postoji
mogucnost da drugi komentiraju.

Kako ljudi ostavljaju komentare, njihovi komentari se **automatski dodaju u skriptu za
reprodukciju** s istim pomakom od ucitavanja stranice, tako da razgovor nastavlja rasti bez
rucnog rada.

Mozete dodatno **moderirati** objavljene komentare, kako biste odabrali koje komentare zelite prikazati
svaki put kada se pokrene skripta za automatsku reprodukciju.

Stranica `Moderate Comments` ce takoder prikazati vremensku oznaku poput `AutoPlay 1hr 2m 30s` pored svakog
komentara umjesto datuma.

Ovo je dostupno samo za automatsku reprodukciju, ne za rucno zakazanu reprodukciju.

### Konfiguracija

Svaki komentar ce biti objavljen **uzivo**. Mozda biste zeljeli razmotriti [ukljucivanje prikaza komentara uzivo odmah](/guide-customizations-and-configuration.html#show-live-right-away).

Mozete saznati vise o formatu uvoza u odjeljku Format uvoza ove dokumentacije.
