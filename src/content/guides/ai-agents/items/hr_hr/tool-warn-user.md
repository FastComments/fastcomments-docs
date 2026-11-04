Alat Warn šalje privatnu DM upozorenje korisniku u vezi s određenim komentarom, a istovremeno bilježi upozorenje u zajedničkoj [memoriji agenta](#agent-memory-system). Ta dva zapisa su atomična - korisnik nikada ne vidi upozorenje koje nije i zabilježeno.

### Zašto postoji

Politika eskalacije platforme je **prvo upozorenje, ban samo ako korisnik ponavlja prekršaj**. Alat Warn je ono što tu politiku čini izvedivom: daje korisniku priliku da ispravi ponašanje, a zapis upozorenja je ono što budući agent pronalazi kada pretražuje memoriju prije razmatranja bana.

Alat također uklanja duplikate: ako je agent već izdao upozorenje istom korisniku za isti komentar, drugo upozorenje nema učinka. Tako LLM koji petlja ili ponovno aktivira na istom komentar ne može zasipati korisnika višestrukim upozorenjima.

### Što ide u upozorenje

Kratka poruka (ograničena na 1000 znakova) koja se korisniku prikazuje kao DM. Snažna upozorenja su:

- **Specifična** - "Osobni napadi na imenovane korisnike nisu dopušteni u ovoj zajednici" bolje je od "vaš komentar je označen."
- **Kratka** - najviše nekoliko rečenica.
- **Izvediva** - recite korisniku što treba promijeniti. "Molimo uredite svoj komentar da uklonite imenovanog korisnika, ili će biti uklonjen."

Poruku ne pišete sami; piše je agent, na temelju [početnog prompta](#personality-prompt) i [smjernica zajednice](#community-guidelines). Vaš je zadatak napisati prompt koji proizvodi dobra upozorenja.

### Kada ga omogućiti

Za svaki agent moderacijskog tipa. Moderator predložak ga omogućuje prema zadanim postavkama.

### Odobrenja

Rjeđe ograničavano od [Ban user](#tool-ban-user). Vrijedi ga ograničiti tijekom prvih tjedana života agenta kako biste mogli uočiti loša upozorenja prije nego što izađu, ali većina operatora uklanja ograničenje kada agent proizvodi pouzdane rezultate.

### Vidi također

- [Ban user](#tool-ban-user) - sljedeći korak u eskalaciji.
- [Agent Memory System](#agent-memory-system) - gdje žive zapisi upozorenja.