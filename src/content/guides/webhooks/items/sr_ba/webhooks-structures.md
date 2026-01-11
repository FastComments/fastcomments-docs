Једина структура која се шаље преко webhook-ова је WebhookComment објекат, описан у TypeScript-у испод.

#### Структура WebhookComment објекта

##### Структура догађаја "create"
Тијело захтјева за догађај "create" је WebhookComment објекат.

##### Структура догађаја "update"
Тијело захтјева за догађај "update" је WebhookComment објекат.

##### Структура догађаја "delete"
Тијело захтјева за догађај "delete" је WebhookComment објекат.

    Промјена од 14. новембра 2023.
    Претходно је тијело захтјева за догађај "delete" садржало само id коментара. Сада садржи цео коментар у тренутку брисања.


[inline-code-attrs-start title = 'Објекат WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id коментара. **/
    id: string
    /** Id или URL који идентификује нит коментара. Нормализовано. **/
    urlId: string
    /** URL који показује гдје је коментар остављен. **/
    url?: string
    /** Id корисника који је оставио коментар. Ако је SSO, биће са префиксом tenant id. **/
    userId?: string
    /** Е-пошта корисника који је оставио коментар. **/
    commenterEmail?: string
    /** Име корисника које се приказује у видгету за коментаре. Код SSO може бити displayName. **/
    commenterName: string
    /** Сирови текст коментара. **/
    comment: string
    /** Текст коментара након парсирања. **/
    commentHTML: string
    /** Спољни id коментара. **/
    externalId?: string
    /** Id родитељског коментара. **/
    parentId?: string | null
    /** UTC датум када је коментар остављен. **/
    date: UTC_ISO_DateString
    /** Комбинована карма гласова (за - против). **/
    votes: number
    votesUp: number
    votesDown: number
    /** true ако је корисник био пријављен када је коментарисао, или је верификовао коментар, или ако је верификовао своју сесију када је коментар остављен. **/
    verified: boolean
    /** Датум када је коментар верификован. **/
    verifiedDate?: number
    /** Ако је модератор означио коментар као прегледан. **/
    reviewed: boolean
    /** Локација или base64 енкодовање аватара. Биће base64 само ако је та вриједност прослијеђена преко SSO. **/
    avatarSrc?: string
    /** Да ли је коментар ручно или аутоматски означен као спам? **/
    isSpam: boolean
    /** Да ли је коментар аутоматски означен као спам? **/
    aiDeterminedSpam: boolean
    /** Да ли коментар садржи слике? **/
    hasImages: boolean
    /** Број странице на којој се коментар налази за смер сортирања "Most Relevant". **/
    pageNumber: number
    /** Број странице на којој се коментар налази за смер сортирања "Oldest First". **/
    pageNumberOF: number
    /** Број странице на којој се коментар налази за смер сортирања "Newest First". **/
    pageNumberNF: number
    /** Да ли је коментар одобрен аутоматски или ручно? **/
    approved: boolean
    /** Локални код (формат: en_us) корисника када је коментар написан. **/
    locale: string
    /** @mentions написани у коментару који су успјешно парсирани. **/
    mentions?: CommentUserMention[]
    /** Домен са којег долази коментар. **/
    domain?: string
    /** Опциона листа id-ева група за модерирање повезаних са овим коментаром. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Када су корисници означени у коментару, информације се чувају у листи названој `mentions`. Сваки објекат у тој листи има сљедећу структуру.

[inline-code-attrs-start title = 'Објекат спомињања у webhook-у'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id корисника. За SSO кориснике, биће са префиксом tenant id. **/
    id: string
    /** Коначни текст @mention тега, укључујући @ симбол. **/
    tag: string
    /** Оригинални текст @mention тега, укључујући @ симбол. **/
    rawTag: string
    /** Који тип корисника је означен. user = FastComments.com налог. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако се корисник одјави са нотификација, ово ће ипак бити постављено на true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP методе

Можете конфигурисати HTTP метод за сваки тип webhook догађаја у административном панелу:

- **Догађај "create"**: POST или PUT (подразумевано: PUT)
- **Догађај "update"**: POST или PUT (подразумевано: PUT)
- **Догађај "delete"**: DELETE, POST или PUT (подразумевано: DELETE)

Пошто сви захтјеви садрже ID, операције креирања и ажурирања су подразумевано идемпотентне (PUT). Понављање истог захтјева за креирање или ажурирање не би требало да створи дупликате на вашој страни.

#### Заглавља захтјева

Сваки webhook захтјев садржи сљедећа заглавља:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ваш API тајни кључ |
| `X-FastComments-Timestamp` | Unix временска ознака (секунде) када је захтјев потписан |
| `X-FastComments-Signature` | HMAC-SHA256 потпис (`sha256=<hex>`) |

Погледајте [Сигурност и API токени](/guides/webhooks/webhooks-api-tokens) за информације о провјери HMAC потписа.