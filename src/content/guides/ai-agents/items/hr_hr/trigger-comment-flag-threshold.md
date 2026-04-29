Pokrreće se kada broj prijava komentara dostigne **točno** konfigurirani prag.

### Potrebna konfiguracija

- **Prag prijava** - cijeli broj >= 1. Okidač se aktivira u trenutku kada `flagCount === flagThreshold`. Ne aktivira se ponovno na naknadne prijave koje prelaze prag.

Ako je prag 3 i tri korisnika prijave komentar, agent se aktivira jednom na trećoj prijavi. Četvrta, peta ili šesta prijava **neće** ga ponovno aktivirati.

### Kontekst koji agent prima

- Komentar koji je prijavljen.
- Opcionalna povijest rasprave / korisnika / kontekst stranice kako je konfigurirano.
- Broj prijava nalazi se u bloku komentara kao `Flag Count: N`.

### Važno

- Okidač se aktivira samo kada komentar prijeđe prag odozdo putem platforminog puta za rukovanje prijavama (gdje je `didIncrement === true`). Izravni zapisi u bazu podataka koji postave `flagCount` na vrijednost praga neće ga aktivirati; prijave iznad praga također ga neće ponovno aktivirati.
- Ne uključuje tko je prijavio komentar - prijave su agentu anonimne. Ako želite vidjeti korisnike koji prijavljuju, dohvatite ih iz vlastitih podataka.
- Kašnjenje okidača (pogledajte [Deferred Triggers](#trigger-deferred-delay)) je *snažno* preporučeno za ovaj okidač - prijave često dolaze u naletima tijekom zapaljene rasprave, a malo kašnjenje dopušta da se situacija smiri prije nego agent djeluje.

### Uobičajene primjene

- **Pregled moderacije** - prijavljeni komentar je kanonični signal "ljudi misle da bi ovo moglo biti loše". [Moderator template](#template-moderator) se prema zadanim postavkama pretplaćuje na ovaj okidač s pragom prijava 3.
- **Proširenje reda za pre-moderaciju** - agent izvrši početni prolaz i ili označi komentar za moderaciju (s `mark_comment_reviewed`) ili ga dodatno eskalira.
- **Protiv brigadiranja** - kombinirajte ovaj okidač s [user history context](#context-options) i dopustite agentu da vidi prethodne zabrane/znakove dupliciranog sadržaja prije nego što djeluje.

### Preporuke za uparivanje

Pretplatite se na **oba** `COMMENT_ADD` i `COMMENT_FLAG_THRESHOLD` ako želite agenta za moderaciju koji uhvati očite slučajeve na prvi pogled i ponovno ocijeni marginalne slučajeve nakon što se prijave nakupine. Dva događaja se aktiviraju neovisno - agent će se pokrenuti dvaput ako je na oba pretplaćen i oba se dogode, ali drugi prolaz vidi sada prijavljeno stanje.