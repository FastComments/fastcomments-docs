Možno je blokirati uporabnike, ki uporabljajo določene ponudnike e‑pošte, z uporabo nadomestnih znakov.

Na primer, če ugotovite, da so vsi komentarji z **@bademail.com** neželeni, lahko preprosto blokirate celotnega ponudnika e‑pošte tako, da v polje za vnos e‑pošte pri dodajanju blokiranega uporabnika vnesete "*@bademail.com".

Opomba: pred znak @ v e‑pošti je znak "*".

### Poddomene

Blokada domene prav tako zajema vse poddomene te domene. Blokiranje `*@bademail.com` prav tako blokira `someone@mail.bademail.com` in `someone@eu.mail.bademail.com`, zato ni potrebno dodajati ločene blokade za vsako poddomeno.

Če želite blokirati le določeno poddomeno, vnesite to poddomeno, na primer `*@mail.bademail.com`. Ta blokada ne vpliva na `someone@bademail.com`.

### Blokiranje domene iz komentarja

Vzorca ne morate vnašati ročno. Ko blokirate uporabnika iz komentarja na strani Moderiranje komentarjev, ima pogovorno okno za blokado potrditveno polje "Blokiraj vse uporabnike @domena", ki ustvari enako `*@domain` blokado za e‑poštno domeno komentatorja.

### Podprte vzorce

Edina podprta oblika nadomestnega znaka je en sam `*` na mestu celotnega dela imena, ki mu sledi `@` in domena. Druge oblike so zavrnjene, ko poskušate shraniti:

- `*@*.bademail.com` ni potreben, ker `*@bademail.com` že zajema poddomene.
- `name*@bademail.com` in `*bademail.com` nista podprta.