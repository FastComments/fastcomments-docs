Objekt `NotificationCount` predstavlja število neprebranih obvestil in metapodatke za uporabnika.

Če ni neprebranih obvestil, za uporabnika ne bo `NotificationCount`.

Objekti `NotificationCount` se ustvarijo samodejno in jih ni mogoče ustvariti prek API-ja. Prav tako potečejo po enem letu.

Število neprebranih obvestil uporabnika lahko počistite z izbrisom njihovega `NotificationCount`.

Struktura objekta `NotificationCount` je naslednja:

[inline-code-attrs-start title = 'Struktura NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // ID uporabnika
    count: number
    createdAt: string // niz datuma
    expireAt: string // niz datuma
}
[inline-code-end]

---