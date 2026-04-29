Alat Warn šalje privatno DM upozorenje korisniku u vezi određenog komentara, i u isto vrijeme beleži upozorenje u zajedničkoj [memoriji agenta](#agent-memory-system). Oba upisa su atomska - korisnik nikada ne vidi upozorenje koje nije i zabeleženo.

### Zašto postoji

Politika eskalacije platforme je **prvo upozorenje, zabrana samo ako korisnik ponovi prekršaj**. Alat Warn je ono što tu politiku čini izvedivom: daje korisniku šansu da ispravi ponašanje, a zapis o upozorenju je ono što budući agent pronađe kada pretražuje memoriju pre nego što razmotri zabranu.

Alat takođe eliminiše duplikate: ako je agent već izdao upozorenje istom korisniku u vezi istog komentara, drugo upozorenje neće imati efekta. Dakle, LLM koji se zaglavi u petlji ili ponovo reaguje na isti komentar ne može spamovati korisnika sa više upozorenja.

### Šta ide u upozorenje

Kratka poruka (ograničena na 1000 karaktera) prikazana korisniku kao DM. Snažna upozorenja su:

- **Specifična** - "Lične uvrede upućene imenovanim korisnicima nisu dozvoljene u ovoj zajednici" bolje zvuči od "vaš komentar je označen."
- **Kratka** - najviše nekoliko rečenica.
- **Akciona** - recite korisniku šta da promeni. "Molimo uredite svoj komentar i uklonite imenovanog korisnika, ili će komentar biti uklonjen."

Vi ne pišete poruku direktno; agent je piše, na osnovu [početnog prompta](#personality-prompt) i [smernica zajednice](#community-guidelines). Vaš posao je da napišete prompt koji proizvodi dobra upozorenja.

### Kada ga dozvoliti

Za bilo kog agenta koji obavlja moderaciju. Šablon Moderator to podrazumeva prema zadanim postavkama.

### Odobrenja

Ređe se stavlja pod kontrolu nego [Ban user](#tool-ban-user). Vredno je kontrolisati ga tokom prvih par nedelja rada agenta kako biste mogli uočiti loša upozorenja pre nego što budu poslata, ali većina operatera uklanja kontrolu kada agent počne da daje pouzdane rezultate.

### Vidi takođe

- [Ban user](#tool-ban-user) - sledeći korak u eskalaciji.
- [Sistem memorije agenta](#agent-memory-system) - gde se čuvaju zapisi upozorenja.