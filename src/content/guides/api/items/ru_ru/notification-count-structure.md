Объект `NotificationCount` представляет количество непрочитанных уведомлений и метаданные пользователя.

Если непрочитанных уведомлений нет, для пользователя не будет объекта `NotificationCount`.

Объекты `NotificationCount` создаются автоматически и не могут быть созданы через API. Они также истекают через год.

Вы можете сбросить количество непрочитанных уведомлений пользователя, удалив его объект `NotificationCount`.

Структура объекта `NotificationCount` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // идентификатор пользователя
    count: number
    createdAt: string // строковое представление даты
    expireAt: string // строковое представление даты
}
[inline-code-end]

---