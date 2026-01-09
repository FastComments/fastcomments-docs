Objekat `NotificationCount` predstavlja broj nepročitanih obaveštenja i metapodatke za korisnika.

Ako nema nepročitanih obaveštenja, za korisnika neće postojati `NotificationCount`.

`NotificationCount` objekti se kreiraju automatski i ne mogu se kreirati putem API-ja. Takođe ističu nakon jedne godine.

Možete obrisati broj nepročitanih obaveštenja korisnika brisanjem njihovog `NotificationCount`.

Struktura za `NotificationCount` objekat je sljedeća:

[inline-code-attrs-start title = 'Struktura NotificationCount objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // ID korisnika
    count: number
    createdAt: string // datum kao string
    expireAt: string // datum isteka kao string
}
[inline-code-end]

---