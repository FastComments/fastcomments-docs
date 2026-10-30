The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Структура на обекта WebhookComment

##### Структура на събитието „Create“
The "create" event request body is a WebhookComment object.

##### Структура на събитието „Update“
The "update" event request body is a WebhookComment object.

##### Структура на събитието „Delete“
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Промяна от 14 ноември 2023 г.
Преди тялото на заявката за събитие „delete“ съдържаше само идентификатора на коментара. Сега съдържа целия коментар към момента на изтриване.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

Всеки ключ винаги присъства в тялото. Когато коментарът няма стойност за дадено поле, тялото съдържа `null` (или `false` за булеви стойности и `[]` за списъци), така че формата на доставката никога не се променя от един коментар към друг.

[inline-code-attrs-start title = 'Обектът WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификаторът на коментара. **/
    id: string
    /** Идентификаторът или URL, който идентифицира нишката на коментара. Нормализиран. **/
    urlId: string
    /** URL‑то, което указва къде е оставен коментарът. **/
    url: string | null
    /** Потребителският идентификатор, който е оставил коментара. При SSO е с префикс на идентификатора на наемателя. **/
    userId: string | null
    /** Имейлът на потребителя, който е оставил коментара. **/
    commenterEmail: string | null
    /** Името на потребителя, което се показва в уиджета за коментари. При SSO може да бъде displayName. **/
    commenterName: string
    /** Необработен текст на коментара. **/
    comment: string
    /** Текст на коментара след парсиране. **/
    commentHTML: string
    /** Външен идентификатор на коментара. **/
    externalId: string | null
    /** Идентификаторът на родителския коментар. **/
    parentId: string | null
    /** UTC датата, когато е оставен коментарът. **/
    date: UTC_ISO_DateString
    /** Комбиниран карма (гласове нагоре - надолу). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Истина, ако потребителят е бил влязъл, когато е коментирал, или е верифицирал коментара, или ако е верифицирал сесията си, когато коментарът е оставен. **/
    verified: boolean
    /** UTC датата, когато коментарът е верифициран. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Ако модератор е маркирал коментара като прегледан. **/
    reviewed: boolean
    /** Местоположението или base64 кодирането на аватара. Ще бъде base64 само ако това е стойността, предадена с SSO. **/
    avatarSrc: string | null
    /** Дали коментарът е маркиран като спам ръчно или автоматично? **/
    isSpam: boolean
    /** Дали коментарът е маркиран автоматично като спам? **/
    aiDeterminedSpam: boolean
    /** Има ли изображения в коментара? **/
    hasImages: boolean
    /** Номерът на страницата, на която се намира коментарът за сортиране „Най-релевантни“. **/
    pageNumber: number | null
    /** Номерът на страницата, на която се намира коментарът за сортиране „Най-стари първи“. **/
    pageNumberOF: number | null
    /** Номерът на страницата, на която се намира коментарът за сортиране „Най-нови първи“. **/
    pageNumberNF: number | null
    /** Дали коментарът е одобрен автоматично или ръчно? **/
    approved: boolean
    /** Кодът на локала (формат: en_us) на потребителя, когато е написан коментарът. **/
    locale: string | null
    /** @споменаванията, написани в коментара, които са успешно парсирани. Празно, ако няма такива. **/
    mentions: CommentUserMention[]
    /** Домейнът, от който е коментарът. **/
    domain: string | null
    /** Идентификаторите на модерационните групи, свързани с този коментар. Празно, ако няма такива. **/
    moderationGroupIds: string[]
    /** Значките, показвани до името на коментатора в този коментар, към момента на създаване на събитието. Празно, ако няма такива. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list has the following structure.

Когато потребители са споменати в коментар, информацията се съхранява в списък, наречен `mentions`. Всеки обект в този списък има следната структура.

[inline-code-attrs-start title = 'Обектът Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Потребителският идентификатор. За SSO потребители, това ще има вашия идентификатор на наемателя като префикс. **/
    id: string
    /** Финалният текст на @споменаване, включително символа @. **/
    tag: string
    /** Оригиналният текст на @споменаване, включително символа @. **/
    rawTag: string
    /** Какъв тип потребител е бил споменат. user = акаунт FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако потребителят се откаже от известия, това все пак ще бъде зададено като true. **/
    sent: boolean
}
[inline-code-end]

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

Всеки елемент в списъка `badges` има следната структура. Вижте [ръководството за значки](/guide-badges.html) за типове значки.

[inline-code-attrs-start title = 'Обектът Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Идентификаторът на значката, присвоена на този потребител. **/
    id: string
    /** Типът на значката. **/
    type: number
    /** Описанието на значката, показвано при задържане на мишката върху нея. **/
    description: string
    /** Текстът, показван на значката. **/
    displayLabel?: string | null
    /** Изображението, показвано за значката, ако използва такова. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Допълнителен CSS клас, приложен към значката. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP методи

You can configure the HTTP method for each webhook event type in the admin panel:

Можете да конфигурирате HTTP метода за всеки тип уебкук събитие в администраторския панел:

- **Create Event**: POST или PUT (по подразбиране: PUT)
- **Update Event**: POST или PUT (по подразбиране: PUT)
- **Delete Event**: DELETE, POST или PUT (по подразбиране: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

Тъй като всички заявки съдържат идентификатор, операциите Create и Update са идемпотентни по подразбиране (PUT). Повтарянето на една и съща заявка за Create или Update не трябва да създава дублирани обекти от ваша страна.

#### Заглавки на заявката

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Вашият API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.

Вижте [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) за информация относно проверката на HMAC подписа.