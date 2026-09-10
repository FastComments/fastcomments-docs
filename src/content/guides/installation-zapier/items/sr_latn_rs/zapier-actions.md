## Akcije i Pretrage

Akcije kreiraju podatke u FastComments; pretrage pronalaze podatke kako bi ih kasniji korak mogao koristiti. Svaka akcija poziva FastComments REST API i troši iste API kredite koliko bi poziv koštao iz vašeg koda: jedan kredit po pozivu, osim ako nije drugačije navedeno.

## Kreiranje komentara

Objavljuje komentar na stranici.

| Polje | Obavezno | Napomene |
|-------|----------|----------|
| Page URL ID | Da | URL ID koji widget za komentare koristi na stranici. Komentari su grupisani po njemu. |
| Page URL | Da | Puni URL stranice, koristi se u e‑mail obaveštenjima. |
| Comment | Da | Telo komentara u FastComments markdownu. |
| Commenter Name | Da | Imena su jedinstvena po e‑mailu, pa ponovno korišćenje imena sa drugačijim e‑mailom ne uspeva. |
| Commenter Email | Ne | Korisnik se kreira za e‑mail kada još ne postoji. |
| User ID | Ne | Postojeći SSO ID korisnika. Ima prednost nad imenom i e‑mailom. |
| Parent Comment ID | Ne | Postavite da biste objavili odgovor. |
| Approved, Verified | Ne | Oba podrazumevano su true. Neodobreni komentari ostaju skriveni dok se ne moderiraju. |
| Posted At | Ne | Podrazumevano je sada. |
| Avatar URL, Page Title, Locale | Ne | Lokalizacija podrazumevano je `en_us`. |
| Show Live In Widget | Ne | Gura komentar gledaocima u realnom vremenu. Košta 2 kredita umesto 1. |
| Run Spam Check, Send Emails | Ne | Isključeno podrazumevano. |

## Kreiranje stranice

Kreira zapis stranice pre nego što na njoj postoji bilo koji komentar, kako bi mogla biti izlistana i ograničena. Prima URL ID, naslov, URL i opcionalno SSO grupne ID‑ove kojima je dozvoljeno da je vide.

## Kreiranje SSO korisnika

Kreira korisnika za jedinstvenu prijavu (single sign‑on). Prima vaš sopstveni ID korisnika, korisničko ime i e‑mail, plus opcionalno prikazno ime, prikaznu oznaku, avatar, veb sajt, grupne ID‑ove, i zastavice za obaveštenja i privatnost. Administrativne uloge ne mogu biti dodeljene putem Zapiera.

## Kreiranje objave u feedu

Kreira objavu u FastComments feedu iz HTML sadržaja, sa opcionalnim naslovom, autorom, oznakama i jednim pregledom linka.

## Kreiranje hash taga

Kreira hash tag koji komentatori mogu koristiti, sa opcionalnim URL‑om na koji vodi.

## Označavanje komentara

Označava komentar za pregled moderatora. Dostavite ID korisnika koji vrši označavanje, ili ostavite prazno da označite kao Zapier integracija.

## Pretrage

| Pretraga | Ulaz | Vraća |
|----------|------|-------|
| Find Comment | Comment ID | Komentar, ili ništa. |
| Find SSO User | Email | SSO korisnik, ili ništa. |
| Find Page | URL ID | Stranica, ili ništa. |

Pretraga koja ne pronađe ništa ne uzrokuje grešku u Zap‑u. Kombinujte pretragu sa kreiranjem u Zapier‑ovom režimu „find or create“ da biste kreirali stranicu ili korisnika kada nedostaje.