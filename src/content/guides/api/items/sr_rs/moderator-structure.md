Objekat `Moderator` predstavlja konfiguraciju za moderatora.

Постоје три типа модератора:

1. Администраторски корисници који имају ознаку `isCommentModeratorAdmin`.
2. SSO корисници са ознаком `isCommentModeratorAdmin`.
3. Редовни коментатори, или FastComments.com корисници, који су позвани као модератори.

Структура `Moderator` се користи да представи стање модерације за случај употребе `3`.

Ако желите да позовете корисника да буде модератор преко API-ја, користите `Moderator` API тако што ћете креирати `Moderator` и `inviting` њих.

Ако корисник нема FastComments.com налог, мејл са позивом ће им помоћи да се подесе. Ако већ има налог, они ће
добити приступ модерацији вашег tenant-а и поље `userId` објекта `Moderator` ће бити ажурирано да показује на њихов налог. Ви нећете имати API
приступ њиховом налогу, јер у овом случају он припада њима и управља га FastComments.com.

Ако вам је потребно потпуно управљање корисничким налогом, препоручујемо или коришћење SSO-а, или додавање њих као [Корисник тенанта](https://fastcomments.com/auth/my-account/users) и
затим додавање објекта `Moderator` ради праћења њихових статистика.

Структура `Moderator` се може користити као механизам за праћење статистике за случајеве употребе `1` и `2`. Након креирања корисника, додајте `Moderator`
објекат са дефинисаним `userId` и њихове статистике ће бити праћене на [Страница модератора коментара](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Структура за објекат `Moderator` је следећа:

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