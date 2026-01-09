Объект `Moderator` представляет конфигурацию модератора.

Существует три типа модераторов:

1. Пользователи-администраторы, у которых установлен флаг `isCommentModeratorAdmin`.
2. Пользователи SSO с флагом `isCommentModeratorAdmin`.
3. Обычные комментаторы или пользователи FastComments.com, приглашённые в качестве модераторов.

Структура `Moderator` используется для представления состояния модерации в случае использования `3`.

Если вы хотите пригласить пользователя в качестве модератора через API, используйте API `Moderator`, создав объект `Moderator` и отправив им приглашение.

Если у пользователя нет аккаунта FastComments.com, письмо с приглашением поможет им зарегистрироваться. Если у них уже есть аккаунт, им будет предоставлен доступ модерации к вашему tenant, и свойство `userId` объекта `Moderator` будет обновлено, чтобы указывать на их пользователя. У вас не будет доступа к их пользователю через API, поскольку в этом случае он принадлежит им и управляется FastComments.com.

Если вам требуется полный контроль над аккаунтом пользователя, мы рекомендуем либо использовать SSO, либо добавить их как [Tenant User](https://fastcomments.com/auth/my-account/users) и затем добавить объект `Moderator` для отслеживания их статистики.

Структура `Moderator` может использоваться как механизм отслеживания статистики для сценариев использования `1` и `2`. После создания пользователя добавьте объект `Moderator` с заданным `userId`, и их статистика будет отслеживаться на [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Структура объекта `Moderator` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]

---