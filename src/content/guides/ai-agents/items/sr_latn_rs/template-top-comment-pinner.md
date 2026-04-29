**ID šablona:** `top_comment_pinner`

Top Comment Pinner prati komentare prvog nivoa koji pređu prag glasova i fiksira ih - zamenjujući ono što je prethodno bilo fiksirano u istoj temi.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt šablona Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

Instrukcija "do not pin replies" je važna: fiksiranje radi na temama, pa fiksiranje odgovora retko ima smisla. Filter "non-promotional" sprečava agenta da pojačava popularne komentare koji su spam sa linkovima.

### Okidači

- **Komentar pređe prag glasova** (`COMMENT_VOTE_THRESHOLD`, podrazumevani prag glasova: 10).

Okidač se aktivira kada neto glasovi komentara (`up - down`) dostignu konfigurisani prag. Podesite broj na formi za uređivanje u zavisnosti od toga koliko su vaše teme aktivne - 10 je razumnu podrazumevanu vrednost za umereno aktivne sajtove.

### Dozvoljeni alati

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fiksiranje nije destruktivno - može se odmah poništiti - zato se ovaj šablon obično izvršava bez odobrenja.

### Preporučeni dodaci pre puštanja u rad

- **Označite "Uključi roditeljski komentar i prethodne odgovore u istoj temi"** u [Opcije konteksta](#context-options). Bez konteksta teme agent ne može pouzdano reći da li već postoji fiksirani komentar koji treba odfiksirati.
- **Prilagodite prag glasova** vašem sajtu. Na prometnim temama 10 se dešava prečesto; na mirnim temama 10 možda nikada neće biti dostignuto.
- **Razmislite o ograničavanju po URL-u** ako želite fiksirane komentare samo na određenim delovima sajta - npr. na vestima, ali ne na najavama.

### Napomena o duplom fiksiranju

Prompt agenta ga uputjava da najpre odfiksira pre nego što fiksira, ali ako model preskoči taj korak, platforma sama po sebi ne nameće pravilo "jedan fiksirani komentar po temi" (možete imati više). Ako je duplo fiksiranje problem na vašem sajtu, stavite `pin_comment` iza odobrenja i pregledajte svaki slučaj - ili napišite precizniji prompt.