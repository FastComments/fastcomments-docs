**ID predloška:** `top_comment_pinner`

Top Comment Pinner nadzire komentare najviše razine koji prijeđu prag glasova i prikvači ih - zamjenjujući ono što je prethodno bilo prikvačeno na istoj niti.

Ugrađeni prompt nalaže agentu da preskoči odgovore (prikvačivanje radi na nitima, pa je prikvačivanje odgovora rijetko korisno) i da filtrira promotivni sadržaj (tako agent ne bi pojačavao popularni spam s linkovima).

### Triggers

- **Komentar prijeđe prag glasova** (`COMMENT_VOTE_THRESHOLD`, zadani prag glasova: 10).

Okidač se aktivira kada neto glasovi komentara (`up - down`) dosegnu konfigurirani prag. Prilagodite broj na obrascu za uređivanje ovisno o tome koliko su vaše niti aktivne - 10 je razuman zadani izbor za umjereno aktivne stranice.

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Prikvačivanje nije destruktivno - može se odmah poništiti - pa ovaj predložak obično radi bez odobrenja.

### Recommended additions before going live

- **Označite "Uključi roditeljski komentar i prethodne odgovore u istoj niti"** u [Opcije konteksta](#context-options). Bez konteksta niti agent ne može pouzdano utvrditi postoji li već prikvačeni komentar koji treba ukloniti.
- **Prilagodite prag glasova** svojoj stranici. Na prometnim nitima 10 se događa prečesto; na mirnim nitima 10 možda nikada neće biti dostignuto.
- **Razmislite o ograničavanju po URL-u** ako želite prikvačene komentare samo u određenim odjeljcima vaše stranice - npr. niti s vijestima, ali ne i niti s najavama.

### Note on duplicate pinning

Prompt agenta nalaže mu da prvo ukloni prikvačenje prije nego što prikvači, ali ako model propusti taj korak, platforma sama po sebi ne provodi pravilo "jedan prikvačeni po niti" (možete imati više). Ako je duplicirano prikvačivanje problem na vašoj stranici, stavite `pin_comment` iza odobrenja i pregledajte svaki slučaj - ili napišite stroži prompt.