Объект `Moderator` представляет конфигурацию модератора.

Существует три типа модераторов:

1. Администраторы, у которых установлен флаг `isCommentModeratorAdmin`.
2. Пользователи SSO с флагом `isCommentModeratorAdmin`.
3. Обычные комментаторы, или пользователи FastComments.com, приглашённые в модераторы.

Структура `Moderator` используется для представления состояния модерации в случае использования `3`.

Если вы хотите пригласить пользователя стать модератором через API, используйте API `Moderator`, создав `Moderator` и пригласив его.

Если у пользователя нет аккаунта на FastComments.com, письмо с приглашением поможет ему настроиться. Если у него уже есть аккаунт, ему будет предоставлен доступ модератора к вашему тенанту и поле `userId` объекта `Moderator` будет обновлено, чтобы указывать на его пользователя. Вы не будете иметь доступа к их пользователю через API, так как в этом случае он принадлежит им и управляется FastComments.com.

Если вам требуется полный контроль над учётной записью пользователя, мы рекомендуем либо использовать SSO, либо добавить их в качестве [Tenant User](https://fastcomments.com/auth/my-account/users) и затем добавить объект `Moderator` для отслеживания их статистики.

Структуру `Moderator` можно использовать как механизм отслеживания статистики для случаев использования `1` и `2`. После создания пользователя добавьте объект `Moderator` с указанным `userId`, и их статистика будет отслеживаться на странице [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Структура объекта `Moderator` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура объекта Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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