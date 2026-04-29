The **Smernice zajednice** polje na formi za uređivanje je opciona polja teksta politike koja se uključuje u kontekstualnu poruku uloge korisnika pri svakom pokretanju ovog agenta. Obeleženo je kao nepouzdan tekst (isto obeležavanje koje platforma primenjuje na tela komentara i drugi sadržaj koji dostavljaju korisnici), tako da ga model tretira kao referencu pravila, a ne kao sistemsku instrukciju. To je zvanično mesto za zapisivanje "koje ponašanje je dozvoljeno, a koje nije na ovom sajtu" kako bi agent primenjivao ta pravila dosledno.

### Kako se razlikuje od početnog prompta

- **Početni prompt** - uloga agenta i stil donošenja odluka. "Vi ste moderator. Preferirajte upozorenje umesto zabrane."
- **Smernice zajednice** - pravila vaše zajednice, formulisana kao politika. "Nema ličnih napada. Nema promotivnih linkova sa naloga starijih manje od 24 sata. Off-topic komentari mogu biti uklonjeni ako je nit podgrejana."

Oba ulaze u isti kontekstni prozor, ali ulaze na različitim slojevima - početni prompt je deo sistemske uloge, dok je dokument smernica obeležen tekst unutar kontekstualne poruke uloge korisnika. Razdvajanje olakšava uređivanje kada želite da ažurirate jedno, a da ne morate ponovo čitati drugo.

### Šta je dobar dokument smernica

Kratak, specifičan, i napisan od strane čoveka. Liste su bolje od proze:

[inline-code-attrs-start title = 'Primer smernica zajednice'; type='text' inline-code-attrs-end]
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

Agent primenjuje ovo pri svakom pokretanju. Ako promenite smernice, promena stupa na snagu pri sledećem okidaču - prethodna izvršenja se ne re-evaluiraju retroaktivno.

### Šta ovde ne treba stavljati

- **Instrukcije za format izlaza** ("odgovori u HTML", "koristi emoji"). To pripada u [početni prompt](#personality-prompt).
- **Lokalizovan tekst.** Dokument smernica, kao i prompt, je **samo na engleskom** iz istog razloga - mašinski prevod može promeniti ponašanje agenta bez upozorenja. Ako imate politike koje se razlikuju po lokalu, napišite ih sve na engleskom u ovom jednom dokumentu i strukturirajte dokument kao "za stranice na nemačkom jeziku: ..."
- **Dugački citati spoljnih politika.** Parafrazirajte. Dugačak kontekst košta tokene pri svakom pokretanju.
- **PII ili tajne.** Ovaj tekst se šalje provajderu LLM-a pri svakom pokretanju.

### Dužina

Polje je ograničeno na **4000 karaktera** (primenjuje se i na formi i na ruti za čuvanje). Trošak tokena pri svakom pokretanju proporcionalan je dužini, tako da je čak i unutar limita nekoliko stotina reči obično dovoljno. Ako vaše politike zajednice zauzimaju mnogo strana, sažmite delove koji su agentu potrebni u skraćenoj verziji posebno za ovo polje.

### Verzije

Nema ugrađene istorije verzija za dokument smernica - poslednja sačuvana vrednost je ta koju agent koristi. Ako želite istoriju, kopirajte dokument u svoj sistem za praćenje pre svakog većeg izmena. [Refine Prompts](#refining-prompts) tok može da zabeleži promene *početnog prompta* ali ne vrši verzionisanje dokumenta smernica.

---