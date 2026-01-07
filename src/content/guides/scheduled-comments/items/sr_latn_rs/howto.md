Koriscenje FastComments Zakazanih komentara je jednostavno. Prvo, zelecete [uvesti komentare ovde](https://fastcomments.com/auth/my-account/manage-data/import-scheduled).

Ovoj stranici se takode moze pristupiti putem `Manage Data->Schedule Comments`.

### Rucno reprodukovani komentari

*Za rucno* reprodukovane komentare (morate rucno pritisnuti Play), imate opciju da pocnete reprodukovati komentare. Ovo ce reprodukovati komentare na svakoj stranici koju ste definisali u CSV
datoteci, sa kasnjenjem izmedju svakog komentara zasnovanim na kasnjenju koje ste naveli.

Ovo je korisno kada imate zakazan webinar uzivo koji pocinje u odredjeno vreme. Kada webinar pocne, samo pritisnite Play
na kontrolnoj tabli.

### Automatska reprodukcija komentara

*Za automatski* reprodukovane komentare, komentari se reprodukuju pri svakom ucitavanju stranice za svakog korisnika.

Ovo je korisno za scenarije gde video ili drugi sadrzaj pocinje od pocetka sa svakim ucitavanjem.

### Dinamicki rastuca reprodukcija

Svaki put kada se skripta za automatsku reprodukciju pokrene za korisnika - pri ucitavanju stranice - jos uvek postoji
mogucnost da drugi komentarisu.

Kako ljudi ostavljaju komentare, njihovi komentari se **automatski dodaju u skriptu za
reprodukciju** sa istim pomakom od ucitavanja stranice, tako da razgovor nastavlja da raste bez
rucnog rada.

Mozete dodatno **moderirati** objavljene komentare, kako biste odabrali koje komentare zelite da prikazete
svaki put kada se pokrene skripta za automatsku reprodukciju.

Stranica `Moderate Comments` ce takodje prikazati vremensku oznaku poput `AutoPlay 1hr 2m 30s` pored svakog
komentara umesto datuma.

Ovo je dostupno samo za automatsku reprodukciju, ne za rucno zakazanu reprodukciju.

### Konfiguracija

Svaki komentar ce biti objavljen **uzivo**. Mozda biste zeleli da razmotrite [ukljucivanje prikaza komentara uzivo odmah](/guide-customizations-and-configuration.html#show-live-right-away).

Mozete saznati vise o formatu uvoza u odeljku Format uvoza ove dokumentacije.
