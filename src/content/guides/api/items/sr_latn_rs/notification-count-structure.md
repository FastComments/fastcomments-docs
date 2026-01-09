Objekat `NotificationCount` predstavlja broj nepročitanih obaveštenja i metapodatke za korisnika.

Ako nema nepročitanih obaveštenja, za korisnika neće postojati `NotificationCount`.

`NotificationCount` objekti se automatski kreiraju i ne mogu se kreirati preko API-ja. Takođe ističu nakon jedne godine.

Možete obrisati broj nepročitanih obaveštenja korisnika tako što ćete izbrisati njihov `NotificationCount`.

Struktura objekta `NotificationCount` je sledeća:

[inline-code-attrs-start title = 'Struktura NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // id korisnika
    count: number
    createdAt: string // datum kao string
    expireAt: string // datum kao string
}
[inline-code-end]

---