Обектът `Moderator` представлява конфигурация за модератор.

Има три типа модератори:

1. Администраторски потребители, които имат флага `isCommentModeratorAdmin`.
2. SSO потребители с флага `isCommentModeratorAdmin`.
3. Обикновени коментиращи или FastComments.com потребители, които са поканени като модератори.

Структурата `Moderator` се използва за представяне на състоянието на модерация за случай на употреба `3`.

Ако искате да поканите потребител да бъде модератор чрез API, използвайте API `Moderator`, като създадете `Moderator` и го `поканите`.

Ако потребителят няма акаунт в FastComments.com, имейлът с покана ще му помогне да се настрои. Ако вече има акаунт, ще му бъде
даден достъп за модерация до вашия tenant и `userId` на обекта `Moderator` ще бъде актуализиран, за да сочи към неговия потребител. Няма да имате API
достъп до техния потребител, тъй като в този случай той принадлежи на самите тях и се управлява от FastComments.com.

Ако изисквате пълно управление на акаунта на потребителя, препоръчваме или да използвате SSO, или да ги добавите като [Tenant потребител](https://fastcomments.com/auth/my-account/users) и
след това да добавите обект `Moderator`, за да проследявате техните статистики.

Структурата `Moderator` може да се използва като механизъм за проследяване на статистики за случаи на употреба `1` и `2`. След създаване на потребителя добавете обект `Moderator`
с дефиниран `userId` и техните статистики ще бъдат проследявани на [страницата с модератори на коментари](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Структурата на обекта `Moderator` е следната:

[inline-code-attrs-start title = 'Структура на Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
