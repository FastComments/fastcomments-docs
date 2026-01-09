Објекат `NotificationCount` представља број непрочитаних обавештења и метаподатке за корисника.

Ако нема непрочитаних обавештења, за тог корисника неће постојати `NotificationCount`.

Објекти `NotificationCount` се креирају аутоматски и не могу се креирати путем API-ja. Такође истичу након једне године.

Можете обрисати број непрочитаних обавештења корисника брисањем њиховог `NotificationCount`.

Структура објекта `NotificationCount` је следећа:

[inline-code-attrs-start title = 'Структура NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // ID корисника
    count: number
    createdAt: string // низ који представља датум
    expireAt: string // низ који представља датум
}
[inline-code-end]

---