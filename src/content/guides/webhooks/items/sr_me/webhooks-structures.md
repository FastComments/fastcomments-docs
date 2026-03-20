Једина структура која се шаље преко webhook-ова је објекат WebhookComment, приказан у TypeScript-у испод.

#### Структура објекта WebhookComment

##### Структура догађаја "create"
Тело захтјева за догађај "create" је објекат WebhookComment.

##### Структура догађаја "update"
Тело захтјева за догађај "update" је објекат WebhookComment.

##### Структура догађаја "delete"
Тело захтјева за догађај "delete" је објекат WebhookComment.

    Измјена од 14. новембра 2023.
    Раније је тело захтјева за догађај "delete" садржавало само id коментара. Сада садржи цијели коментар у тренутку брисања.


[inline-code-attrs-start title = 'Објекат WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификатор коментара. **/
    id: string
    /** Ид или URL који идентификује нит коментара. Нормализовано. **/
    urlId: string
    /** URL који показује гдје је коментар остављен. **/
    url?: string
    /** Ид корисника који је оставио коментар. Ако је SSO, има префикс tenant id. **/
    userId?: string
    /** Е-пошта корисника који је оставио коментар. **/
    commenterEmail?: string
    /** Име корисника које се приказује у видгету коментара. Са SSO, може бити displayName. **/
    commenterName: string
    /** Сиров текст коментара. **/
    comment: string
    /** Текст коментара након парсирања. **/
    commentHTML: string
    /** Спољни id коментара. **/
    externalId?: string
    /** Ид родитељског коментара. **/
    parentId?: string | null
    /** UTC датум када је коментар остављен. **/
    date: UTC_ISO_DateString
    /** Комбинована карма (за - против) гласова. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ако је корисник био пријављен када је коментарисао, или ако је његов коментар верификован, или ако је потврдио сесију када је коментар остављен. **/
    verified: boolean
    /** Датум када је коментар верификован. **/
    verifiedDate?: number
    /** Ако је модератор означио коментар као прегледан. **/
    reviewed: boolean
    /** Локација, или base64 енкодинг, аватара. Биће base64 само ако је та вриједност прослијеђена уз SSO. **/
    avatarSrc?: string
    /** Да ли је коментар ручно или аутоматски означен као спам? **/
    isSpam: boolean
    /** Да ли је коментар аутоматски означен као спам? **/
    aiDeterminedSpam: boolean
    /** Да ли у коментару има слика? **/
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
    /** @mentions написани у коментару који су успешно парсирани. **/
    mentions?: CommentUserMention[]
    /** Домен одакле је коментар. **/
    domain?: string
    /** Опциона листа id-ева група модерације повезаних са овим коментаром. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Када су корисници означени у коментару, информација се чува у листи званој `mentions`. Сваки објекат у тој листи има сљедећу структуру.

[inline-code-attrs-start title = 'Објекат помена у webhook-у'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Ид корисника. За SSO кориснике, биће са префиксом tenant id. **/
    id: string
    /** Коначни @mention таг текст, укључујући @ симбол. **/
    tag: string
    /** Оригинални @mention таг текст, укључујући @ симбол. **/
    rawTag: string
    /** Који тип корисника је тагован. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако се корисник одлучи да не прими обавештења, ово ће ипак бити постављено на true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP методе

Можете конфигурисати HTTP метод за сваку врсту webhook догађаја у админ панелу:

- **Догађај "create"**: POST или PUT (подразумјевано: PUT)
- **Догађај "update"**: POST или PUT (подразумјевано: PUT)
- **Догађај "delete"**: DELETE, POST или PUT (подразумјевано: DELETE)

Пошто сви захтјеви садрже ИД, операције Create и Update су по дифолту идемпотентне (PUT). Понављање истог Create или Update захтјева не би требало да створи дупликате објеката на вашој страни.

#### Заглавља захтјева

Сваки webhook захтјев укључује сљедећа заглавља:

| Заглавље | Опис |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш API секрет |
| `X-FastComments-Timestamp` | Unix временски жиг (секунде) када је захтјев потписан |
| `X-FastComments-Signature` | HMAC-SHA256 потпис (`sha256=<hex>`) |

Погледајте [Сигурност и API токени](/guide-webhooks.html#webhooks-api-tokens) за информације о верификацији HMAC потписа.

---