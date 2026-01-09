Объект `NotificationCount` представляет количество непрочитанных уведомлений и метаданные для пользователя.

Если нет непрочитанных уведомлений, для пользователя не будет объекта `NotificationCount`.

Объекты `NotificationCount` создаются автоматически и не могут быть созданы через API. Они также истекают через год.

Вы можете очистить количество непрочитанных уведомлений пользователя, удалив его объект `NotificationCount`.

Структура объекта `NotificationCount` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура объекта NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // идентификатор пользователя
    count: number
    createdAt: string // строка с датой
    expireAt: string // строка с датой
}
[inline-code-end]

---