The Warn tool šalje privatnu DM poruku upozorenja korisniku u vezi konkretnog komentara, i istovremeno beleži upozorenje u deljenoj [memoriji agenta](#agent-memory-system). Dva zapisa su atomska - korisnik nikada ne vidi upozorenje koje nije takođe zabeleženo.

### Zašto postoji

Politika eskalacije platforme je **prvo upozorenje, zabrana samo ako korisnik ponovi prekršaj**. Warn tool je ono što tu politiku čini primenljivom: daje korisniku šansu da ispravi ponašanje, a zapis upozorenja je ono što budući agent nalazi kada pretražuje memoriju pre nego što razmotri zabranu.

Alat takođe uklanja duplikate: ako je agent već izdao upozorenje istom korisniku za isti komentar, drugo upozorenje ne radi ništa. Dakle, LLM koji se petlja ili opet pokreće na istom komentaru ne može spamovati korisnika sa više upozorenja.

### Šta ide u upozorenje

Kratka poruka (ograničena na 1000 karaktera) prikazana korisniku kao DM. Snažna upozorenja su:

- **Specifično** - "Lični napadi na imenovane korisnike nisu dozvoljeni u ovoj zajednici" je bolja poruka od "vaš komentar je prijavljen."
- **Kratko** - najviše nekoliko rečenica.
- **Konkretno** - recite korisniku šta da promeni. "Molimo uredite svoj komentar da uklonite imenovanog korisnika, inače će biti uklonjen."

Vi ne pišete poruku sami; agent to radi, zasnovano na [početnom promptu](#personality-prompt) i [smernicama zajednice](#community-guidelines). Vaš posao je da napišete prompt koji proizvodi dobra upozorenja.

### Kada dozvoliti

Za bilo kog agenta za moderaciju. Šablon Moderator ga podrazumevano omogućava.

### Odobrenja

Ređe je ograničeno odobrenjem nego [Ban user](#tool-ban-user). Vredno je zahtevati odobrenje tokom prvih nedelja rada agenta kako biste uočili loša upozorenja pre nego što se pošalju, ali većina operatora ukloni to ograničenje kada agent počne davati pouzdan izlaz.

### Vidi takođe

- [Ban user](#tool-ban-user) - naredni korak u eskalaciji.
- [Sistem memorije agenta](#agent-memory-system) - mesto gde žive zapisi upozorenja.