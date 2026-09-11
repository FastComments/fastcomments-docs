## Akcije i Pretrage

Akcije kreiraju podatke u FastComments; pretrage pronalaze podatke kako bi ih kasniji korak mogao koristiti. Svaka akcija poziva FastComments REST API i troši iste API kredite koliko bi poziv koštao iz vašeg koda: jedan kredit po pozivu, osim ako nije drugačije naznačeno.

## Kreiraj Komentar

Objavljuje komentar na stranici.

| Polje | Obavezno | Napomene |
|-------|----------|----------|
| ID URL-a stranice | Da | ID URL-a koji widget za komentare koristi na stranici. Komentari su grupisani po njemu. |
| URL stranice | Da | Pun URL stranice, koristi se u obaveštajnim emailovima. |
| Komentar | Da | Sadržaj komentara u FastComments markdown-u. |
| Ime komentatora | Da | Imena su jedinstvena po email-u, pa ponovno korišćenje imena sa drugim email-om ne uspeva. |
| Email komentatora | Ne | Korisnik se kreira za email ako još ne postoji. |
| ID korisnika | Ne | Postojeći SSO ID korisnika. Ima prednost nad imenom i email-om. |
| ID roditeljskog komentara | Ne | Postavite da biste objavili odgovor. |
| Odobreno, Verifikovano | Ne | Oba podrazumevano su true. Neodobreni komentari ostaju skriveni dok se ne moderiraju. |
| Objavljeno u | Ne | Podrazumevano je sada. |
| URL avatara, Naslov stranice, Lokalitet | Ne | Lokalitet podrazumevano je `en_us`. |
| Prikaži uživo u widgetu | Ne | Gura komentar gledaocima u realnom vremenu. Košta 2 kredita umesto 1. |
| Pokreni proveru spama, Pošalji emailove | Ne | Podrazumevano isključeno. |

## Kreiraj ili Ažuriraj Stranicu

Kreira zapis stranice pre nego što na njoj postoji bilo koji komentar, kako bi mogla biti listirana i ograničena. Prima ID URL-a, naslov, URL i opcionalno SSO ID-ove grupa kojima je dozvoljeno da je vide. Ako stranica sa tim ID-jem URL-a već postoji, ažurira se poljima koja su data, tako da Zap može više puta da se izvrši za istu stranicu.

## Kreiraj ili Ažuriraj SSO Korisnika

Kreira korisnika za jedinstvenu prijavu (single sign-on). Prima vaš sopstveni ID korisnika, korisničko ime i email, plus opcionalno prikazno ime, prikaznu oznaku, avatar, veb sajt, ID-ove grupa, i zastavice za obaveštenja i privatnost. Ako korisnik sa tim ID-jem već postoji, umesto toga se ažurira. Administrativne uloge ne mogu biti dodeljene iz Zapiera.

## Kreiraj Feed Post

Kreira post u FastComments feed-u iz HTML sadržaja. ID autora korisnika je obavezan (FastComments ili SSO ID korisnika); naslov, tagovi i jedan pregled linka su opcionalni.

## Kreiraj ili Ažuriraj Hash Tag

Kreira hash tag koji komentatori mogu da koriste, sa opcionalnim URL-om na koji vodi. Ako tag već postoji, umesto toga se ažurira.

## Obeleži Komentar

Obeležava komentar za pregled moderatora. ID korisnika koji obeležava je obavezan; ID autora koji se vrati iz Kreiraj Komentar funkcije funkcioniše.

## Pretrage

| Pretraga | Ulaz | Vraća |
|----------|------|-------|
| Pronađi Komentar | ID Komentara | Komentar, ili ništa. |
| Pronađi SSO Korisnika | Email | SSO korisnik, ili ništa. |
| Pronađi Stranicu | ID URL-a | Stranica, ili ništa. |

Pretraga koja ne pronađe ništa ne uzrokuje grešku u Zap-u. Pronađi SSO Korisnika i Pronađi Stranicu nude Zapier-ovu opciju „kreiraj ako ne postoji“, koja izvršava odgovarajuću kreaciju kada ništa nije pronađeno.