## Akcije i pretrage

Akcije kreiraju podatke u FastComments; pretrage traže podatke kako bi ih kasniji korak mogao koristiti. Svaka akcija poziva FastComments REST API i troši iste API kredite koliko bi poziv koštao iz vašeg koda: jedan kredit po pozivu, osim ako nije drugačije naznačeno.

## Kreiraj komentar

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
| Objavljeno u | Ne | Podrazumevano je trenutno vreme. |
| URL avatara, Naslov stranice, Lokalitet | Ne | Lokalitet podrazumevano je `en_us`. |
| Prikaži uživo u widgetu | Ne | Gura komentar gledaocima u realnom vremenu. Košta 2 kredita umesto 1. |
| Pokreni proveru spama, Šalji emailove | Ne | Podrazumevano isključeno. |

## Kreiraj stranicu

Kreira zapis stranice pre nego što na njoj postoji bilo koji komentar, kako bi mogla biti izlistana i ograničena. Prima ID URL-a, naslov, URL i opcionalno SSO ID-ove grupa kojima je dozvoljeno da je vide.

## Kreiraj SSO korisnika

Kreira korisnika za jedinstvenu prijavu (single sign-on). Prima vaš sopstveni ID korisnika, korisničko ime i email, plus opcionalno prikazno ime, prikaznu oznaku, avatar, veb sajt, ID-ove grupa, i zastavice za obaveštenja i privatnost. Administrativne uloge ne mogu biti dodeljene iz Zapiera.

## Kreiraj objavu u feedu

Kreira objavu u FastComments feedu iz HTML sadržaja. ID korisnika autora je obavezan (FastComments ili SSO ID korisnika); naslov, tagovi i jedan pregled linka su opcionalni.

## Kreiraj hash tag

Kreira hash tag koji komentatori mogu koristiti, uz opcionalni URL na koji vodi. Tagovi su jedinstveni po nalogu, pa Zap koji kreira jedan pri svakom pokretanju treba nešto jedinstveno u tagu.

## Oznaci komentar

Označava komentar za pregled moderatora. ID korisnika koji vrši označavanje je obavezan; ID autora vraćen iz Kreiraj komentar funkcije funkcioniše.

## Pretrage

| Pretraga | Ulaz | Vraća |
|----------|------|-------|
| Pronađi komentar | ID komentara | Komentar, ili ništa. |
| Pronađi SSO korisnika | Email | SSO korisnik, ili ništa. |
| Pronađi stranicu | ID URL-a | Stranica, ili ništa. |

Pretraga koja ne pronađe ništa ne uzrokuje grešku u Zap-u. Kombinujte pretragu sa kreiranjem u Zapier-ovom režimu “find or create” da biste kreirali stranicu ili korisnika kada nedostaje.