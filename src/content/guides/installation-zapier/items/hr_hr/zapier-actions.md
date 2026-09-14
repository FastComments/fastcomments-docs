## Radnje i pretrage

Radnje stvaraju podatke u FastComments; pretrage traže podatke kako bi ih kasniji korak mogao koristiti. Svaka radnja poziva FastComments REST API i troši iste API kredite koliko bi poziv koštao iz vašeg vlastitog koda: jedan kredit po pozivu, osim ako nije drugačije navedeno.

## Stvaranje komentara

Objavljuje komentar na stranici.

| Polje | Obavezno | Napomene |
|-------|----------|----------|
| ID URL-a stranice | Da | ID URL-a koji widget za komentare koristi na stranici. Komentari su grupirani po njemu. |
| URL stranice | Da | Puni URL stranice, koristi se u obavijesnim e‑mailovima. |
| Komentar | Da | Tijelo komentara u FastComments markdownu. |
| Ime komentatora | Da | Imena su jedinstvena po e‑mailu, pa ponovno korištenje imena s drugim e‑mailom ne uspijeva. |
| E‑mail komentatora | Ne | Korisnik se kreira za taj e‑mail ako još ne postoji. |
| ID korisnika | Ne | Postojeći SSO ID korisnika. Ima prednost nad imenom i e‑mailom. |
| ID nadređenog komentara | Ne | Postavite za objavu odgovora. |
| Odobreno, Potvrđeno | Ne | Oba zadano su postavljena na true. Neodobreni komentari ostaju skriveni dok se ne moderiraju. |
| Objavljeno u | Ne | Zadano je sada. |
| URL avatara, Naslov stranice, Lokalitet | Ne | Lokalitet zadano je `en_us`. |
| Prikaži uživo u widgetu | Ne | Gura komentar gledateljima u stvarnom vremenu. Košta 2 kredita umjesto 1. |
| Pokreni provjeru spama, Šalji e‑mailove | Ne | Zadano isključeno. |

## Stvaranje ili ažuriranje stranice

Stvara zapis stranice prije nego što na njoj postoji bilo koji komentar, kako bi se mogla izlistati i ograničiti. Prima ID URL-a, naslov, URL i opcionalno SSO ID‑ove grupa kojima je dozvoljeno vidjeti je. Ako stranica s tim ID‑om URL‑a već postoji, ažurira se s navedenim poljima, tako da Zap može više puta pokrenuti za istu stranicu.

## Stvaranje ili ažuriranje SSO korisnika

Stvara korisnika za jedinstvenu prijavu (single sign‑on). Prima vaš vlastiti ID korisnika, korisničko ime i e‑mail, plus opcionalno prikazno ime, prikaznu oznaku, avatar, web‑stranicu, ID‑ove grupa te oznake za obavijesti i privatnost. Ako korisnik s tim ID‑om već postoji, umjesto toga se ažurira. Administrativne uloge ne mogu se dodijeliti putem Zapiera.

## Stvaranje objave u feedu

Stvara objavu u FastComments feedu iz HTML sadržaja. ID korisnika autora je obavezan (FastComments ili SSO ID korisnika); naslov, oznake i jedan pregled poveznice su opcionalni.

## Stvaranje ili ažuriranje hash oznake

Stvara hash oznaku koju komentatori mogu koristiti, s opcionalnim URL‑om na koji vodi. Ako oznaka već postoji, umjesto toga se ažurira.

## Označavanje komentara

Označava komentar za pregled moderatora. ID korisnika koji označava je obavezan; ID autora vraćen od Stvaranja komentara funkcionira.

## Pretrage

| Pretraga | Ulaz | Vraća |
|----------|------|-------|
| Pronađi komentar | ID komentara | Komentar, ili ništa. |
| Pronađi SSO korisnika | E‑mail | SSO korisnik, ili ništa. |
| Pronađi stranicu | ID URL‑a | Stranica, ili ništa. |

Pretraga koja ne pronađe ništa ne uzrokuje neuspjeh Zapa. Pretrage Pronađi SSO korisnika i Pronađi stranicu nude Zapierovu opciju „stvori ako ne postoji“, koja pokreće odgovarajuće stvaranje kada se ništa ne pronađe.