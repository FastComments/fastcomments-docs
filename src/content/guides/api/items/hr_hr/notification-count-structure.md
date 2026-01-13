Objekt `NotificationCount` predstavlja broj nepročitanih obavijesti i metapodatke za korisnika.

Ako nema nepročitanih obavijesti, za korisnika neće postojati `NotificationCount`.

`NotificationCount` objekti se stvaraju automatski i ne mogu se stvoriti putem API-ja. Također istječu nakon jedne godine.

Možete obrisati broj nepročitanih obavijesti korisnika brisanjem njihovog `NotificationCount`.

Struktura `NotificationCount` objekta je sljedeća:

[inline-code-attrs-start title = 'Struktura NotificationCount objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // ID korisnika
    count: number
    createdAt: string // string datuma
    expireAt: string // string datuma
}
[inline-code-end]

---