Polje **Smjernice zajednice** na obrascu za uređivanje je neobavezan blok teksta pravila koji se uključuje u poruku konteksta korisničke uloge pri svakom pokretanju ovog agenta. Označen je kao nepouzdani tekst (istim označavanjem koje platforma primjenjuje na tijela komentara i drugi korisnički sadržaj), tako da ga model tretira kao referencu pravila, a ne kao sistemske upute. To je kanonsko mjesto za zapisivanje "koje je ponašanje dopušteno, a koje nije na ovom sajtu" kako bi agent dosljedno primjenjivao ta pravila.

### Kako se razlikuje od početnog prompta

- **Početni prompt** - uloga agenta i stil donošenja odluka. "Ti si moderator. Daj prednost upozorenju umjesto banovanja."
- **Smjernice zajednice** - pravila vaše zajednice, u jeziku pravila. "Nema osobnih napada. Nema promotivnih poveznica s računa starijih od 24 sata. Komentari izvan teme mogu se ukloniti ako je nit razgorena."

Oba ulaze u isti kontekstni prozor, ali ulaze na različitim razinama - početni prompt je dio sistemske uloge, a dokument smjernica je označeni tekst unutar poruke konteksta korisničke uloge. Ta podjela olakšava uređivanje kada želite ažurirati jedno bez ponovnog čitanja drugoga.

### Kako izgleda dobar dokument smjernica

Kratak, specifičan dokument koji je napisao čovjek. Popisi bolje funkcioniraju od proznog teksta:

[inline-code-attrs-start title = 'Primjer smjernica zajednice'; type='text' inline-code-attrs-end]
[inline-code-start]
Dopušteno:
- Suštinsko neslaganje, čak i oštro izraženo.
- Poveznice na izvorne izvore, čak i s novih računa.
- Sporedne digresije ako glavna nit to dopušta.

Nije dopušteno:
- Osobni napadi na konkretne, imenovane korisnike.
- Doxxing ili dijeljenje privatnih informacija.
- Koordinirana promotivna aktivnost (više komentara koji guraju istu vanjsku poveznicu).
- Komentari koji postoje samo radi ometanja rasprave.

Granični slučajevi:
- Snažan jezik bez ciljane osobe. Dopušteno ako nije usmjeren na osobu.
- Političke teme izvan teme stranice. Izvan teme; prvo upozoriti, ne uklanjati osim ako se nastavi.
[inline-code-end]

Agent ovo primjenjuje pri svakom pokretanju. Ako promijenite smjernice, promjena stupa na snagu pri sljedećem okidanju - prošla pokretanja se ne preispituju retroaktivno.

### Što ne stavljati ovdje

- **Upute za formatiranje izlaza** ("odgovori u HTML-u", "koristi emojije"). To pripada u [početni prompt](#personality-prompt).
- **Lokalizirani tekst.** Dokument smjernica, kao i prompt, je **samo na engleskom** iz istog razloga - strojno prevođenje može tiho promijeniti ponašanje agenta. Ako imate pravila koja variraju po jeziku, napišite ih sve na engleskom u ovom jednom dokumentu i strukturirajte dokument kao "za stranice na njemačkom jeziku: ..."
- **Dugi citati vanjskih pravila.** Parafrazirajte. Dugi kontekst košta tokene pri svakom pokretanju.
- **Osobni podaci ili tajne.** Ovaj tekst se šalje davatelju LLM usluge pri svakom pokretanju.

### Duljina

Polje je ograničeno na **4000 znakova** (provjerava i obrazac i ruta za spremanje). Trošak tokena pri svakom pokretanju proporcionalan je duljini, pa je čak i unutar ograničenja nekoliko stotina riječi obično dovoljno. Ako se pravila vaše zajednice protežu na više stranica, sažmite dijelove koje agent treba u sažetak namijenjen posebno ovom polju.

### Verzioniranje

Ne postoji ugrađena povijest verzija za dokument smjernica - agent koristi zadnju spremljenu vrijednost. Ako želite povijest, kopirajte dokument u vlastiti sustav praćenja prije svake veće izmjene. Tok [Refine Prompts](#refining-prompts) može zapisati promjene *početnog prompta*, ali ne verzionira dokument smjernica.