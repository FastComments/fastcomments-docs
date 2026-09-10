## Actions and Searches

Akcije stvaraju podatke u FastComments; pretrage traže podatke kako bi ih kasniji korak mogao koristiti. Svaka akcija poziva FastComments REST API i troši iste API kredite koliko bi poziv koštao iz vašeg vlastitog koda: jedan kredit po pozivu, osim ako nije navedeno.

## Create Comment

Posts a comment on a page.

| Polje | Obavezno | Napomene |
|-------|----------|----------|
| Page URL ID | Da | URL ID koji widget za komentare koristi na stranici. Komentari su grupirani po njemu. |
| Page URL | Da | Puni URL stranice, koristi se u obavijesnim e‑mailovima. |
| Comment | Da | Tijelo komentara u FastComments markdownu. |
| Commenter Name | Da | Imena su jedinstvena po e‑mailu, pa ponovno korištenje imena s drugačijim e‑mailom ne uspijeva. |
| Commenter Email | Ne | Korisnik se kreira za e‑mail ako još ne postoji. |
| User ID | Ne | Postojeći SSO korisnički ID. Ima prednost nad imenom i e‑mailom. |
| Parent Comment ID | Ne | Postavite za objavu odgovora. |
| Approved, Verified | Ne | Oba su po zadanom postavljena na true. Neodobreni komentari ostaju skriveni dok se ne moderiraju. |
| Posted At | Ne | Zadano je trenutno vrijeme. |
| Avatar URL, Page Title, Locale | Ne | Locale po zadanom je `en_us`. |
| Show Live In Widget | Ne | Gura komentar gledateljima u stvarnom vremenu. Košta 2 kredita umjesto 1. |
| Run Spam Check, Send Emails | Ne | Po zadanom isključeno. |

## Create Page

Stvara zapis stranice prije nego što na njoj postoji bilo koji komentar, kako bi se mogla izlistati i ograničiti. Prima URL ID, naslov, URL i opcionalno SSO grupne ID‑ove kojima je dozvoljeno vidjeti je.

## Create SSO User

Stvara korisnika za jednokratnu prijavu (single sign-on). Prima vaš vlastiti korisnički ID, korisničko ime i e‑mail, plus opcionalno prikazno ime, prikaznu oznaku, avatar, web‑stranicu, grupne ID‑ove, te zastavice za obavijesti i privatnost. Administrativne uloge ne mogu se dodijeliti putem Zapiera.

## Create Feed Post

Stvara objavu u FastComments feedu iz HTML sadržaja, s opcionalnim naslovom, autorom, oznakama i jednim pregledom poveznice.

## Create Hash Tag

Stvara hash oznaku koju komentatori mogu koristiti, s opcionalnim URL‑om na koji vodi.

## Flag Comment

Označava komentar za pregled moderatora. Navedite ID korisnika koji označava, ili ostavite prazno da označite kao Zapier integracija.

## Searches

| Pretraga | Ulaz | Vraća |
|----------|------|-------|
| Find Comment | Comment ID | Komentar, ili ništa. |
| Find SSO User | Email | SSO korisnik, ili ništa. |
| Find Page | URL ID | Stranica, ili ništa. |

Pretraga koja ne pronađe ništa ne uzrokuje neuspjeh Zapa. Kombinirajte pretragu s kreiranjem u Zapierovom načinu „find or create“ (pronađi ili kreiraj) kako biste stvorili stranicu ili korisnika kada nedostaje.