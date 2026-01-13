Et `NotificationCount`-objekt repræsenterer antallet af ulæste notifikationer og metadata for en bruger.

Hvis der ikke er nogen ulæste notifikationer, vil der ikke være en `NotificationCount` for brugeren.

`NotificationCount`-objekter oprettes automatisk og kan ikke oprettes via API'et. De udløber også efter et år.

Du kan rydde en brugers antal ulæste notifikationer ved at slette deres `NotificationCount`.

Strukturen for `NotificationCount`-objektet er som følger:

[inline-code-attrs-start title = 'NotificationCount Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]
