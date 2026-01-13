Об'єкт `Moderator` представляє конфігурацію для модератора.

Існує три типи модераторів:

1. Користувачі-адміністратори, які мають прапорець `isCommentModeratorAdmin`.
2. Користувачі SSO з прапорцем `isCommentModeratorAdmin`.
3. Звичайні коментатори або користувачі FastComments.com, яких запрошують як модераторів.

Структура `Moderator` використовується для представлення стану модерації для випадку використання `3`.

Якщо ви хочете запросити користувача стати модератором через API, використайте API `Moderator`, створивши `Moderator` і `inviting` їх.

Якщо у користувача немає облікового запису FastComments.com, електронний лист із запрошенням допоможе їм налаштуватись. Якщо вони вже мають обліковий запис, вони
отримають доступ для модерації до вашого tenant, і поле `userId` об'єкта `Moderator` буде оновлено, щоб вказувати на їхнього користувача. Ви не матимете API
доступу до їхнього користувача, оскільки в цьому випадку він належить їм і управляється FastComments.com.

Якщо вам потрібне повне управління обліковим записом користувача, ми рекомендуємо або використовувати SSO, або додати їх як a [Tenant User](https://fastcomments.com/auth/my-account/users) і
потім додати об'єкт `Moderator` для відстеження їхніх статистик.

Структура `Moderator` може використовуватись як механізм відстеження статистики для випадків використання `1` та `2`. Після створення користувача, додайте `Moderator`
об'єкт з визначеним `userId`, і їхні статистики будуть відстежуватись на сторінці [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

The structure for the `Moderator` object is as follows:

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