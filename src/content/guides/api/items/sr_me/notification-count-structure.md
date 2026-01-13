A `NotificationCount` object predstavlja број непрочитаних обавјештења и метаподатке за корисника.

Ако нема непрочитаних обавјештења, за корисника неће постојати `NotificationCount`.

`NotificationCount` објекти се креирају аутоматски и не могу се креирати преко API-ја. Такођер истичу након једне године.

Можете очистити број непрочитаних обавјештења корисника брисањем његовог `NotificationCount`.

Структура за објекат `NotificationCount` је следећа:

[inline-code-attrs-start title = 'Структура NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // ид корисника
    count: number
    createdAt: string // датум у облику низа
    expireAt: string // датум у облику низа
}
[inline-code-end]

---