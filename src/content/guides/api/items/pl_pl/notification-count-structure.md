Obiekt `NotificationCount` reprezentuje liczbę nieprzeczytanych powiadomień i metadane dla użytkownika.

Jeśli nie ma nieprzeczytanych powiadomień, nie będzie istniał żaden `NotificationCount` dla użytkownika.

Obiekty `NotificationCount` są tworzone automatycznie i nie można ich tworzyć za pomocą API. Wygasają również po roku.

Możesz wyczyścić liczbę nieprzeczytanych powiadomień użytkownika, usuwając jego `NotificationCount`.

Struktura obiektu `NotificationCount` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // id użytkownika
    count: number
    createdAt: string // łańcuch daty
    expireAt: string // łańcuch daty
}
[inline-code-end]

---