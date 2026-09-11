## Radnje i pretrage

Radnje stvaraju podatke u FastComments; pretrage traže podatke kako bi ih kasniji korak mogao koristiti. Svaka radnja poziva FastComments REST API i troši iste API kredite koliko bi taj poziv koštao iz vašeg koda: jedan kredit po pozivu, osim ako nije drugačije navedeno.

## Stvori komentar

Objavljuje komentar na stranici.

| Polje | Obavezno | Napomene |
|-------|----------|----------|
| ID URL-a stranice | Da | ID URL-a koji widget za komentare koristi na stranici. Komentari su grupirani po njemu. |
| URL stranice | Da | Puni URL stranice, koristi se u obavijesnim e‑mailovima. |
| Komentar | Da | Tijelo komentara u FastComments markdownu. |
| Ime komentatora | Da | Imena su jedinstvena po e‑mailu, pa ponovno korištenje imena s drugačijim e‑mailom ne uspije. |
| E‑mail komentatora | Ne | Korisnik se stvara za taj e‑mail ako još ne postoji. |
| ID korisnika | Ne | Postojeći SSO ID korisnika. Ima prednost nad imenom i e‑mailom. |
| ID nadređenog komentara | Ne | Postavite za objavu odgovora. |
| Odobreno, Potvrđeno | Ne | Oboje je zadano na true. Neodobreni komentari ostaju skriveni dok se ne moderiraju. |
| Objavljeno u | Ne | Zadatno je sada. |
| URL avatara, Naslov stranice, Lokalitet | Ne | Lokalitet je zadano `en_us`. |
| Prikaži uživo u widgetu | Ne | Guranje komentara gledateljima u stvarnom vremenu. Košta 2 kredita umjesto 1. |
| Pokreni provjeru spama, Pošalji e‑mailove | Ne | Zadatno isključeno. |

## Stvori stranicu

Stvara zapis stranice prije nego što na njoj postoji bilo koji komentar, kako bi se mogla prikazati i ograničiti. Prima ID URL-a, naslov, URL i opcionalno SSO ID‑ove grupa kojima je dozvoljen pristup.

## Stvori SSO korisnika

Stvara korisnika za jednokratnu prijavu (single sign‑on). Prima vaš vlastiti ID korisnika, korisničko ime i e‑mail, plus opcionalno prikazno ime, prikaznu oznaku, avatar, web‑stranicu, ID‑ove grupa te oznake za obavijesti i privatnost. Administrativne uloge ne mogu se dodijeliti putem Zapiera.

## Stvori objavu u feedu

Stvara objavu u FastComments feedu iz HTML sadržaja. ID korisnika autora je obavezan (FastComments ili SSO ID korisnika); naslov, oznake i jedan pregled poveznice su opcionalni.

## Stvori hash oznaku

Stvara hash oznaku koju komentatori mogu koristiti, s opcionalnim URL‑om na koji vodi. Oznake su jedinstvene po računu, pa Zap koji stvara jednu pri svakom pokretanju treba nešto jedinstveno u oznaci.

## Označi komentar

Označava komentar za pregled od strane moderatora. Potreban je ID korisnika koji vrši označavanje; ID autora vraćen iz Stvori komentar funkcionira.

## Pretrage

| Pretraga | Ulaz | Vraća |
|----------|------|-------|
| Pronađi komentar | ID komentara | Komentar, ili ništa. |
| Pronađi SSO korisnika | E‑mail | SSO korisnika, ili ništa. |
| Pronađi stranicu | ID URL-a | Stranicu, ili ništa. |

Pretraga koja ne pronađe ništa ne uzrokuje neuspjeh Zapa. Kombinirajte pretragu s kreiranjem u Zapier‑ovom načinu „find or create“ kako biste stvorili stranicu ili korisnika kada nedostaje.