Об'єкт `Moderator` представляє конфігурацію модератора.

Існує три типи модераторів:

1. Користувачі-адміністратори, які мають прапорець `isCommentModeratorAdmin`.
2. SSO-користувачі з прапорцем `isCommentModeratorAdmin`.
3. Звичайні коментатори або користувачі FastComments.com, яких запрошено як модераторів.

Структура `Moderator` використовується для представлення стану модерації варіанту використання `3`.

Якщо ви хочете запросити користувача стати модератором через API, використовуйте API `Moderator`, створивши об'єкт `Moderator` і надіславши йому `запрошення`.

Якщо користувач не має облікового запису FastComments.com, лист із запрошенням допоможе йому налаштувати обліковий запис. Якщо обліковий запис уже існує, користувач отримає доступ до модерації вашого тенанта, а поле `userId` об'єкта `Moderator` буде оновлено, щоб вказувати на його користувача. Ви не матимете API-доступу до цього користувача, оскільки в цьому випадку він належить самому користувачу та керується FastComments.com.

Якщо вам потрібне повне керування обліковим записом користувача, ми рекомендуємо або використовувати SSO, або додати користувача як [Tenant User](https://fastcomments.com/auth/my-account/users), а потім створити об'єкт `Moderator` для відстеження його статистики.

Структура `Moderator` може використовуватися як механізм відстеження статистики для варіантів використання `1` і `2`. Після створення користувача додайте об'єкт `Moderator` із зазначеним `userId`, і його статистика відстежуватиметься на [Сторінці модераторів коментарів](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Структура об'єкта `Moderator` виглядає так:

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