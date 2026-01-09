Объект `Notification` представляет уведомление для пользователя.

`Notification` объекты создаются автоматически и не могут быть созданы через API. Они также истекают через год.
Уведомления не могут быть удалены. Однако их можно обновлять, устанавливая `viewed` в `false`, и вы можете выполнять запросы по полю `viewed`.

Пользователь также может отписаться от уведомлений для конкретного комментария, установив `optedOut` в уведомлении в `true`. Вы можете снова подписаться, установив его в `false`.

Существуют разные типы уведомлений — смотрите `relatedObjectType` и `type`.

Механизмы создания уведомлений довольно гибкие и могут быть вызваны множеством сценариев (см. `NotificationType`).

На сегодняшний день наличие `Notification` не означает, что электронное письмо отправлено или должно быть отправлено. Вместо этого уведомления
используются для ленты уведомлений и связанных интеграций.

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
    /** Если кто-то ответил где-либо в ветке обсуждения (включая ответы на ответы) темы, в которой вы оставили комментарий. **/
    RepliedTransientChild = 1,
    /** Если ваш комментарий был оценён положительно. **/
    VotedMyComment = 2,
    /** Если на корне страницы, на которую вы подписаны, оставлен новый комментарий. **/
    SubscriptionReplyRoot = 3,
    /** Если кто-то прокомментировал ваш профиль. **/
    CommentedOnProfile = 4,
    /** Если у вас есть личное сообщение. **/
    DirectMessage = 5,
    /** TrialLimits предназначен только для пользователей тенанта. **/
    TrialLimits = 6,
    /** Если вас упомянули. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`. **/
    userId?: string
    /** При работе с SSO вам нужно учитывать только `userId`. **/
    anonUserId?: string
    /** urlId почти всегда определён. Он является необязательным только для уведомлений на уровне тенанта, которые случаются редко. **/
    urlId?: string
    /** URL кешируется для быстрого перехода к источнику уведомления. **/
    url?: string
    /** Заголовок страницы кешируется для быстрого ознакомления с источником уведомления. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Например, идентификатор комментария. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // строка даты
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName и fromUserAvatarSrc кешируются здесь для быстрого отображения уведомления. Они обновляются при обновлении пользователя. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Установите это в true, чтобы перестать получать уведомления для этого объекта. **/
    optedOut?: boolean
}
[inline-code-end]

---