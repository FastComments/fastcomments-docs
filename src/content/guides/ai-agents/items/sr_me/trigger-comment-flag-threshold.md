Aktivira se kada broj oznaka (flag) na komentaru dostigne **tačno** konfigurisan prag.

### Obavezna konfiguracija

- **Flag threshold** - ceo broj >= 1. Okidač se aktivira u trenutku kada `flagCount === flagThreshold`. Ne aktivira se ponovo na narednim oznakama nakon što je prag prekoračen.

Ako je prag 3 i tri korisnika prijave komentar, agent će se aktivirati jednom na trećoj prijavi. Četvrta, peta ili šesta prijava neće je ponovo aktivirati.

### Kontekst koji agent dobija

- Prijavljeni komentar.
- Opcioni kontekst teme / istorije korisnika / stranice kako je konfigurisan.
- Broj flagova je u bloku komentara kao `Flag Count: N`.

### Važno

- Okidač se aktivira samo kada komentar pređe prag odozdo kroz put obrade flagova platforme (gde je `didIncrement === true`). Direktna upisivanja u bazu koja postave `flagCount` na vrednost praga ga ne aktiviraju; oznake preko praga takođe ga ne ponovo aktiviraju.
- Ne uključuje ko je prijavio komentar - prijave su anonimne za agenta. Ako želite da vidite korisnike koji su prijavili, dobijte ih iz sopstvenih podataka.
- Kašnjenje okidača (pogledajte [Deferred Triggers](#trigger-deferred-delay)) se *snažno* preporučuje za ovaj okidač - prijave često pristignu u talasima tokom žustre teme, i malo odlaganje omogućava da se slika slegne pre nego što agent reaguje.

### Uobičajene upotrebe

- **Pregled moderacije** - prijavljeni komentar je kanonski signal "ljudi misle da ovo možda nije u redu". Šablon [Moderator template](#template-moderator) se podrazumevano pretplaćuje na ovaj okidač sa pragom od 3.
- **Dopuna reda za pred-moderaciju** - agent izvršava inicijalnu proveru i ili označava komentar za moderaciju (sa `mark_comment_reviewed`) ili ga dalje eskalira.
- **Protiv brigadiranja** - kombinuje ovaj okidač sa [kontekst istorije korisnika](#context-options) i omogućite agentu da vidi prethodne zabrane/znakove dupliranog sadržaja pre nego što deluje.

### Preporuke za uparivanje

Pretplatite se na **oba** `COMMENT_ADD` i `COMMENT_FLAG_THRESHOLD` ako želite agenta za moderaciju koji uhvati očigledne slučajeve na prvi pogled i ponovo proceni granične slučajeve kada se nakupi više prijava. Dva događaja se aktiviraju nezavisno - agent će se pokrenuti dvaput ako je na oba pretplaćen i oba se dogode, ali drugo pokretanje vidi sadašnje stanje sa prijavom.

---