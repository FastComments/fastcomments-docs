Једина структура која се шаље преко вебхукова је објекат WebhookComment, описан у TypeScript-у испод.

#### Структура објекта WebhookComment

##### Структура догађаја "Create"
Тело захтева за догађај "create" је објекат WebhookComment.

##### Структура догађаја "Update"
Тело захтева за догађај "update" је објекат WebhookComment.

##### Структура догађаја "Delete"
Тело захтева за догађај "delete" је објекат WebhookComment.

    Промена од 14. новембра 2023.
    Пре тога, тело захтева за догађај "delete" садржало је само id коментара. Сада садржи цео коментар у тренутку брисања.


[inline-code-attrs-start title = 'Објекат WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ИД коментара. **/
    id: string
    /** ИД или URL који идентификује нит коментара. Нормализовано. **/
    urlId: string
    /** URL који показује где је коментар остављен. **/
    url?: string
    /** ИД корисника који је оставио коментар. Ако је SSO, са префиксом tenant id. **/
    userId?: string
    /** Е-пошта корисника који је оставио коментар. **/
    commenterEmail?: string
    /** Име корисника које се приказује у видгету за коментаре. Са SSO, може бити displayName. **/
    commenterName: string
    /** Непроцесирани текст коментара. **/
    comment: string
    /** Текст коментара након парсирања. **/
    commentHTML: string
    /** Спољашњи id коментара. **/
    externalId?: string
    /** ИД родитељског коментара. **/
    parentId?: string | null
    /** UTC датум када је коментар остављен. **/
    date: UTC_ISO_DateString
    /** Сумирани карама (за - против) гласова. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ако је корисник био пријављен када је коментарисао, или ако је верификовао коментар, или ако је верификовао своју сесију када је коментар остављен. **/
    verified: boolean
    /** Датум када је коментар верификован. **/
    verifiedDate?: number
    /** Ако је модератор означио коментар као прегледан. **/
    reviewed: boolean
    /** Локација, или base64 енкодовање, аватара. Биће base64 само ако је та вредност послата са SSO. **/
    avatarSrc?: string
    /** Да ли је коментар ручно или аутоматски означен као спам? **/
    isSpam: boolean
    /** Да ли је коментар аутоматски означен као спам? **/
    aiDeterminedSpam: boolean
    /** Да ли постоје слике у коментару? **/
    hasImages: boolean
    /** Број странице на којој се коментар налази за сортирање "Most Relevant". **/
    pageNumber: number
    /** Број странице на којој се коментар налази за сортирање "Oldest First". **/
    pageNumberOF: number
    /** Број странице на којој се коментар налази за сортирање "Newest First". **/
    pageNumberNF: number
    /** Да ли је коментар одобрен аутоматски или ручно? **/
    approved: boolean
    /** Код локала (формат: en_us) корисника када је коментар написан. **/
    locale: string
    /** @помене написане у коментару које су успешно парсиране. **/
    mentions?: CommentUserMention[]
    /** Домен са којег потиче коментар. **/
    domain?: string
    /** Опциони списак id-ева група модерације повезаних са овим коментаром. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Када су корисници означени у коментару, информације се чувају у списку који се зове `mentions`. Сваки објекат у том списку
има следећу структуру.

[inline-code-attrs-start title = 'Објекат помена Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ИД корисника. За SSO кориснике, овоме ће бити додат префикс tenant id. **/
    id: string
    /** Коначан текст @mention ознаке, укључујући симбол @. **/
    tag: string
    /** Оригинални текст @mention ознаке, укључујући симбол @. **/
    rawTag: string
    /** Каквa врста корисника је означена. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако се корисник одјави са обавештења, ово ће и даље бити постављено на true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP методи

Можете конфигурисати HTTP метод за сваку врсту вебхук догађаја у админ панелу:

- **Create догађај**: POST или PUT (подразумевано: PUT)
- **Update догађај**: POST или PUT (подразумевано: PUT)
- **Delete догађај**: DELETE, POST, или PUT (подразумевано: DELETE)

Пошто сви захтеви садрже ID, операције Create и Update су подразумевано идемпотентне (PUT). Понављање истог Create или Update захтева не би требало да креира дупликат објеката на вашој страни.

#### Заглавља захтева

Сваки вебхук захтев укључује следећа заглавља:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш API тајни кључ |
| `X-FastComments-Timestamp` | Unix временска ознака (у секундама) када је захтев потписан |
| `X-FastComments-Signature` | HMAC-SHA256 потпис (`sha256=<hex>`) |

Погледајте [Безбедност и API токени](/guide-webhooks.html#webhooks-api-tokens) за информације о верификацији HMAC потписа.