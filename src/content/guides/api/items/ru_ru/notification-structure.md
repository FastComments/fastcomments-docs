Объект `Notification` представляет уведомление для пользователя.

Объекты `Notification` создаются автоматически и не могут быть созданы через API. Они также истекают через год.
Уведомления нельзя удалять. Тем не менее их можно обновить, установив `viewed` в `false`, и вы можете выполнять запросы по полю `viewed`.

Пользователь также может отказаться от уведомлений для конкретного комментария, установив `optedOut` в уведомлении в значение `true`. Вы можете снова подписаться, установив его в `false`.

Существует несколько типов уведомлений — смотрите `relatedObjectType` и `type`.

Способы создания уведомлений довольно гибкие и могут быть вызваны многими сценариями (см. `NotificationType`).

На сегодняшний день наличие `Notification` не обязательно означает, что электронное письмо было или должно быть отправлено. Уведомления используются для ленты уведомлений и связанных интеграций.

Структура объекта `Notification` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура объекта Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Если кто-то ответил вам. **/
    RepliedToMe = 0,
    /** Если кто-то ответил в любом месте ветки (даже в потомках) ветки, в которой вы комментировали. **/
    RepliedTransientChild = 1,
    /** Если за ваш комментарий проголосовали за (up-vote). **/
    VotedMyComment = 2,
    /** Если на корне страницы, на которую вы подписаны, оставлен новый комментарий. **/
    SubscriptionReplyRoot = 3,
    /** Если кто-то прокомментировал ваш профиль. **/
    CommentedOnProfile = 4,
    /** Если у вас личное сообщение (DM). **/
    DirectMessage = 5,
    /** TrialLimits предназначен только для пользователей арендатора (tenant). **/
    TrialLimits = 6,
    /** Если вас @упомянули. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`. **/
    userId?: string
    /** При работе с SSO вам нужно учитывать только `userId`. **/
    anonUserId?: string
    /** urlId почти всегда определён. Он является необязательным только для уведомлений уровня арендатора (tenant), которые встречаются редко. **/
    urlId?: string
    /** URL кэшируется для быстрой навигации к источнику уведомления. **/
    url?: string
    /** Заголовок страницы кэшируется для быстрого чтения источника уведомления. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Например, идентификатор комментария. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // строка даты
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName и fromUserAvatarSrc кэшируются здесь для быстрого отображения уведомления. Они обновляются при обновлении пользователя. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Установите в true, чтобы прекратить получение уведомлений для этого объекта. **/
    optedOut?: boolean
}
[inline-code-end]