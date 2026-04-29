**ID predloška:** `top_comment_pinner`

Top Comment Pinner nadgleda komentare na najvišoj razini koji prijeđu prag glasova i prikvači ih - zamjenjujući ono što je prethodno bilo prikvačeno na istoj niti.

### Ugrađeni početni upit

[inline-code-attrs-start title = 'Početni upit predloška Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

Uputa "ne prikvačivati odgovore" je važna: prikvačivanje funkcionira na razini niti, pa prikvačivanje odgovora rijetko ima smisla. Filtriranje "ne-promotivno" sprječava da agent promovira popularan komentar s link-spamom.

### Okidači

- **Komentar prijeđe prag glasova** (`COMMENT_VOTE_THRESHOLD`, zadani prag glasova: 10).

Okidač se aktivira kada neto glasovi komentara (`up - down`) dosegnu konfigurirani prag. Podesite broj u obrascu za uređivanje na temelju toga koliko su vaše niti aktivne - 10 je razuman zadani izbor za umjereno aktivne stranice.

### Dozvoljeni alati

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Prikvačivanje nije destruktivno - može se odmah poništiti - tako da ovaj predložak obično radi bez odobrenja.

### Preporučeni dodaci prije objave

- **Označite "Include parent comment and prior replies in the same thread"** u [Opcije konteksta](#context-options). Bez konteksta niti agent ne može pouzdano utvrditi postoji li već prikvačeni komentar koji treba odvojiti.
- **Prilagodite prag glasova** svojoj stranici. Na prometnim nitima 10 se događa prečesto; na tihim nitima 10 se možda nikada neće dogoditi.
- **Razmislite o ograničavanju po URL-u** ako želite prikvačene komentare samo na određenim dijelovima svoje stranice - na primjer novinske niti, ali ne i niti s najavama.

### Napomena o dupliciranom prikvačivanju

Upit agenta naređuje mu da prvo odvezati prije prikvačivanja, ali ako model preskoči taj korak, sama platforma ne nameće pravilo o jednom prikvačenom komentaru po niti (možete imati više). Ako je duplicirano prikvačivanje problem na vašoj stranici, stavite `pin_comment` iza odobrenja i pregledajte svaki slučaj - ili napišite stroži upit.