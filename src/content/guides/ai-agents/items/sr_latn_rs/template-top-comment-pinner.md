---
**ID šablona:** `top_comment_pinner`

Top Comment Pinner nadgleda komentare prvog nivoa koji pređu prag glasova i zakači ih - zamenjujući ono što je prethodno bilo zakačeno na istoj niti.

Ugrađeni prompt nalaže agentu da preskoči odgovore (zakačivanje funkcioniše na nitima, tako da je zakačivanje odgovora retko korisno) i da filtrira promotivni sadržaj (kako agent ne bi pojačao popularni link-spam).

### Okidači

- **Komentar pređe prag glasova** (`COMMENT_VOTE_THRESHOLD`, podrazumevani prag glasova: 10).

Okidač se aktivira kada net glasova komentara (`up - down`) dostigne konfigurisan prag. Podesite broj na formi za uređivanje u zavisnosti od toga koliko su vaše niti aktivne - 10 je razuman podrazumevani izbor za umereno aktivne sajtove.

### Dozvoljeni alati

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Zakačivanje nije destruktivno - može se odmah poništiti - zato se ovaj šablon obično izvršava bez odobrenja.

### Preporučeni dodaci pre puštanja u rad

- **Označite "Uključi roditeljski komentar i prethodne odgovore u istu nit"** u [Opcije konteksta](#context-options). Bez konteksta nite agent ne može pouzdano reći da li već postoji zakačen komentar koji treba odkačiti.
- **Podesite prag glasova** u skladu sa vašim sajtom. Na zauzetim nitima 10 se dešava prečesto; na mirnim nitima 10 možda nikada neće biti dostignuto.
- **Razmislite o ograničavanju po URL-u** ako želite zakačene komentare samo na određenim sekcijama vašeg sajta - recimo u nitima vesti, ali ne u nitima saopštenja.

### Napomena o duplom zakačivanju

Agentov prompt mu nalaže da prvo odkači pre nego što zakači, ali ako model propusti taj korak sama platforma ne nameće pravilo da samo jedan komentar može biti zakačen po niti (možete imati više njih). Ako je duplo zakačivanje problem na vašem sajtu, stavite `pin_comment` iza odobrenja i pregledajte svaki slučaj - ili napišite stroži prompt.

---