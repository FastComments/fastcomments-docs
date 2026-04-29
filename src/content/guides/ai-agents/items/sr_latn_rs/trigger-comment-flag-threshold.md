Okida se kada broj prijava komentara dostigne **tačno** konfigurisan prag.

### Potrebna konfiguracija

- **Prag za prijave** - celobrojno >= 1. Okidač se aktivira u trenutku kada `flagCount === flagThreshold`. Ne aktivira se ponovo na sledećim prijavama nakon što prag bude premašen.

Ako je prag 3 i tri korisnika prijave komentar, agent se aktivira jednom na trećoj prijavi. Četvrta, peta ili šesta prijava ga **neće** ponovo aktivirati.

### Kontekst koji agent prima

- Prijavljeni komentar.
- Opcioni kontekst teme / istorije korisnika / stranice kako je konfigurisano.
- Broj prijava se nalazi u bloku komentara kao `Flag Count: N`.

### Napomene

- Okidač se aktivira samo kada komentar pređe prag odozdo putem platformskog puta obrade prijava (gde `didIncrement === true`). Direktni zapisi u DB koji postave `flagCount` na vrednost praga ga ne aktiviraju; prijave koje prelaze prag takođe ga ne aktiviraju ponovo.
- Ne uključuje ko je prijavio komentar — prijave su agentu anonimne. Ako želite da vidite korisnike koji su prijavili, izvucite ih iz sopstvenih podataka.
- Snažno se preporučuje odlaganje okidača (pogledajte [Odloženi okidači](#trigger-deferred-delay)) za ovaj okidač - prijave često stižu u naletima tokom žustre teme, i mala odgoda omogućava da se slika slegne pre nego što agent preduzme akciju.

### Uobičajene upotrebe

- **Pregled moderacije** - prijavljeni komentar je kanonični signal „ljudi misle da ovo možda nije u redu“. [Šablon moderatora](#template-moderator) po defaultu se pretplaćuje na ovaj okidač sa pragom prijava 3.
- **Dopunjavanje reda za pre-moderaciju** - agent izvršava početnu proveru i ili označava komentar za moderaciju (sa `mark_comment_reviewed`) ili ga dalje eskalira.
- **Protiv brigadiranja** - kombinuјte ovaj okidač sa [kontekstom istorije korisnika](#context-options) i dozvolite agentu da vidi prethodne zabrane/signale duplikata sadržaja pre nego što deluje.

### Preporuke za kombinovanje

Pretplatite se na **obu** `COMMENT_ADD` i `COMMENT_FLAG_THRESHOLD` ako želite agenta za moderaciju koji uoči očigledne slučajeve na prvi pogled i ponovo proceni upitne slučajeve kada se prijave nagomilaju. Dva događaja se aktiviraju nezavisno - agent će se pokrenuti dva puta ako su oba pretplaćena i oba se aktiviraju, ali drugo pokretanje vidi sadašnje stanje sa prijavom.