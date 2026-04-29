Polje **Smjernice zajednice** na obrascu za uređivanje je opciono polje s tekstom politike koje se uključuje u poruku konteksta korisničke uloge pri svakom pokretanju ovog agenta. Ono je ograničeno kao nepouzdan tekst (isto ograničavanje koje platforma primjenjuje na tijela komentara i drugi sadržaj koji dostavljaju korisnici), pa model tretira to kao referencu politike, a ne kao sistemsku instrukciju. To je kanonsko mjesto za zapisivanje "kakvo ponašanje je i nije dozvoljeno na ovom sajtu" kako bi agent postupao dosljedno.

### Kako se razlikuje od inicijalnog upita

- **Početni upit** - uloga agenta i stil donošenja odluka. "Vi ste moderator. Dajte prednost upozorenju umjesto zabrane."
- **Smjernice zajednice** - pravila vaše zajednice, u jeziku politike. "Nema ličnih napada. Nema promotivnih linkova sa naloga starosti manje od 24 sata. Komentari koji nisu u temi mogu biti uklonjeni ako je nit žustra."

Oba ulaze u isti kontekstni prozor, ali ulaze na različitim slojevima - početni upit je dio sistemske uloge, a dokument smjernica je ograničen tekst unutar poruke konteksta korisničke uloge. Ovo razdvajanje olakšava uređivanje kada želite ažurirati jedno bez ponovnog čitanja drugog.

### Šta je dobar dokument smjernica

Kratak, specifičan, dokument koji je napisao čovjek. Liste funkcionišu bolje od proze:

[inline-code-attrs-start title = 'Primjer smjernica zajednice'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

Agent primjenjuje ovo pri svakom pokretanju. Ako promijenite smjernice, promjena stupa na snagu pri sljedećem okidaču - prethodna pokretanja se ne preispituju retroaktivno.

### Šta ovdje ne stavljati

- **Upute za formatiranje izlaza** ("reply in HTML", "use emoji"). One pripadaju [početnom upitu](#personality-prompt).
- **Lokalizovani tekst.** Dokument smjernica, kao i upit, je **samo na engleskom** iz istog razloga - mašinski prevod može tiho promijeniti ponašanje agenta. Ako imate politike koje variraju po lokaciji, napišite ih sve na engleskom u ovom dokumentu i strukturirajte dokument kao "for German-language pages: ..."
- **Dugački citati spoljašnjih politika.** Parafrazirajte. Dugi kontekst povećava potrošnju tokena pri svakom pokretanju.
- **PII ili tajne.** Ovaj tekst se šalje provajderu LLM-a pri svakom pokretanju.

### Dužina

Polje ima ograničenje od **4000 karaktera** (primjenjuje se i u formi i na ruti za čuvanje). Trošak tokena pri svakom pokretanju je proporcionalan dužini, tako da je čak i unutar limita nekoliko stotina riječi obično dovoljno. Ako vaše politike zajednice zauzimaju mnogo stranica, sažmite dijelove koji su agentu potrebni u specifikaciju namijenjenu upravo ovom polju.

### Verzionisanje

Ne postoji ugrađena istorija verzija za dokument smjernica - agent koristi posljednju sačuvanu vrijednost. Ako želite istoriju, kopirajte dokument u vlastiti sistem za praćenje prije svake veće izmjene. Tok [Refine Prompts](#refining-prompts) može zabilježiti promjene *početnog upita* ali ne vodi verzije dokumenta smjernica.