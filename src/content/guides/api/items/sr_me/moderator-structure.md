A `Moderator` објекат представља конфигурацију за модератора.

Постоје три типа модератора:

1. Администраторски корисници који имају `isCommentModeratorAdmin` ознаку.
2. SSO корисници са `isCommentModeratorAdmin` ознаком.
3. Редовни коментатори, или FastComments.com корисници, који су позвани као модератори.

Структура `Moderator` се користи за представљање стања модерације у случају употребе `3`.

Ако желите да позовете корисника да постане модератор преко API-ја, користите `Moderator` API креирањем `Moderator` и њиховим `inviting`.

Ако корисник нема FastComments.com налог, e-mail позив ће им помоћи да се подесе. Ако већ има налог, добиће приступ модерацији за ваш tenant и `Moderator` објект ће имати ажуриран `userId` да показује на њихов корисник. Нећете имати API приступ њиховом кориснику, јер у том случају он припада њима и управља њим FastComments.com.

Ако вам треба потпуна управљања корисничким налогом, препоручујемо или коришћење SSO, или додавање њих као [Корисник тенанта](https://fastcomments.com/auth/my-account/users) и затим додавање `Moderator` објекта да бисте пратили њихове статистике.

Структура `Moderator` може да се користи као механизам за праћење статистике за случајеве употребе `1` и `2`. Након креирања корисника, додајте `Moderator` објекат са дефинисаним `userId` и њихове статистике ће се пратити на [Страница модератора коментара](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Структура за `Moderator` објекат је следећа:

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