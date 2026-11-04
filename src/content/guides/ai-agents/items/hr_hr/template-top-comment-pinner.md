**ID predloška:** `top_comment_pinner`

Top Comment Pinner prati komentare najviše razine koji prijeđu prag glasova i pribadača ih - zamjenjujući ono što je prethodno bilo pribodano na istoj niti.

Ugrađeni prompt upućuje agenta da preskoči odgovore (pribadača radi na nitima, pa je pribadača odgovora rijetko korisna) i da filtrira promotivni sadržaj (tako agent ne potiče popularan link-spam).

### Okidači

- **Komentar prelazi prag glasova** (`COMMENT_VOTE_THRESHOLD`, zadani prag glasova: 10).

Okidač se aktivira kada neto glasovi komentara (`up - down`) dosegne konfigurirani prag. Prilagodite broj na obrascu za uređivanje ovisno o tome koliko su vaše niti aktivne - 10 je razumna zadana vrijednost za umjereno aktivne stranice.

### Dopušteni alati

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Pribadača je nedestruktivna - može se odmah poništiti - pa ovaj predložak obično radi bez odobrenja.

### Preporučene dopune prije objave

- **Označite "Include parent comment and prior replies in the same thread"** u [Context Options](#context-options). Bez konteksta niti agent ne može pouzdano utvrditi postoji li već pribodan komentar koji treba otipkati.
- **Prilagodite prag glasova** svojoj stranici. Na prometnim nitima 10 se događa prečesto; na mirnim nitima 10 se možda nikada neće dogoditi.
- **Razmislite o ograničavanju po URL-u** ako želite pribodane komentare samo u određenim dijelovima svoje stranice - recimo, na novinskim nitima, ali ne i na nitima s najavama.

### Napomena o dvostrukoj pribadači

Prompt agenta upućuje ga da prvo otipka komentar prije pribadače, ali ako model preskoči taj korak, platforma sama ne nameće pravilo jednog pribodanog komentara po niti (možete imati više njih). Ako je dvostruka pribadača problem na vašoj stranici, postavite `pin_comment` iza odobrenja i pregledajte svaki - ili napišite stroži prompt.