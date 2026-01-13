Обектът `NotificationCount` представлява броя на непрочетените известия и метаданни за потребител.

Ако няма непрочетени известия, няма да има `NotificationCount` за потребителя.

Обектите `NotificationCount` се създават автоматично и не могат да бъдат създавани чрез API. Те също изтичат след една година.

Можете да изчистите броя на непрочетените известия на потребител, като изтриете неговия `NotificationCount`.

Структурата на обекта `NotificationCount` е следната:

[inline-code-attrs-start title = 'Структура на NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]
