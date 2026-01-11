---
Единствената структура, изпращана чрез webhooks, е обектът WebhookComment, описан в TypeScript по-долу.

#### Структура на обекта WebhookComment

##### Структура на събитието "create"
Тялото на заявката за събитието "create" е обект WebhookComment.

##### Структура на събитието "update"
Тялото на заявката за събитието "update" е обект WebhookComment.

##### Структура на събитието "delete"
Тялото на заявката за събитието "delete" е обект WebhookComment.

    Промяна към 14 ноември 2023 г.
    Преди това тялото на заявката за събитието "delete" съдържаше само id на коментара. Сега съдържа пълния коментар в момента на изтриването.


[inline-code-attrs-start title = 'Обектът WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификаторът на коментара. **/
    id: string
    /** Идентификаторът или URL адресът, който идентифицира нишката с коментари. Нормализиран. **/
    urlId: string
    /** URL адресът, който сочи към мястото, където е оставен коментарът. **/
    url?: string
    /** Идентификаторът на потребителя, който е оставил коментара. Ако е SSO, с префикс tenant id. **/
    userId?: string
    /** Имейлът на потребителя, който е оставил коментара. **/
    commenterEmail?: string
    /** Името на потребителя, което се показва в comment widget-а. При SSO може да бъде displayName. **/
    commenterName: string
    /** Суров текст на коментара. **/
    comment: string
    /** Текстът на коментара след парсване. **/
    commentHTML: string
    /** Външен идентификатор на коментара. **/
    externalId?: string
    /** Идентификаторът на родителския коментар. **/
    parentId?: string | null
    /** UTC датата, когато е оставен коментарът. **/
    date: UTC_ISO_DateString
    /** Комбинирана карма (up - down) на гласовете. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Вярно, ако потребителят е бил влязъл, когато е коментирал, ако е верифицирал коментара, или ако е верифицирал сесията си, когато е оставил коментара. **/
    verified: boolean
    /** Дата, когато коментарът е бил верифициран. **/
    verifiedDate?: number
    /** Ако модератор е отбелязал коментара като прегледан. **/
    reviewed: boolean
    /** Местоположението, или base64 кодирането, на аватара. Ще бъде base64 само ако това е стойността, подадена при SSO. **/
    avatarSrc?: string
    /** Коментарът маркиран ли е ръчно или автоматично като спам? **/
    isSpam: boolean
    /** Коментарът автоматично ли беше маркиран като спам? **/
    aiDeterminedSpam: boolean
    /** Има ли изображения в коментара? **/
    hasImages: boolean
    /** Номерът на страницата, на която се намира коментарът за сортиране "Most Relevant". **/
    pageNumber: number
    /** Номерът на страницата, на която се намира коментарът за сортиране "Oldest First". **/
    pageNumberOF: number
    /** Номерът на страницата, на която се намира коментарът за сортиране "Newest First". **/
    pageNumberNF: number
    /** Коментарът одобрен ли е автоматично или ръчно? **/
    approved: boolean
    /** Кодът на локала (формат: en_us) на потребителя, когато е написан коментарът. **/
    locale: string
    /** @mentions, написани в коментара, които са успешно парснати. **/
    mentions?: CommentUserMention[]
    /** Домейнът, от който е коментарът. **/
    domain?: string
    /** Опционален списък с идентификатори на модераторски групи, асоциирани с този коментар. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Когато потребители са маркирани в коментар, информацията се съхранява в списък, наречен `mentions`. Всеки обект в този списък
има следната структура.

[inline-code-attrs-start title = 'Обектът Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Идентификаторът на потребителя. За SSO потребителите това ще има префикс с вашия tenant id. **/
    id: string
    /** Финалният @mention таг текст, включително символа @. **/
    tag: string
    /** Оригиналният @mention таг текст, включително символа @. **/
    rawTag: string
    /** Какъв тип потребител е бил маркиран. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако потребителят се откаже от нотификации, това все пак ще бъде зададено на true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

Можете да конфигурирате HTTP метода за всеки тип webhook събитие в администраторския панел:

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Тъй като всички заявки съдържат ID, операциите Create и Update са идемпотентни по подразбиране (PUT). Повтарянето на една и съща заявка за Create или Update не би трябвало да създаде дублирани обекти от ваша страна.

#### Request Headers

Всяка webhook заявка включва следните заглавки:

| Хедър | Описание |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Вашият API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

Вижте [Сигурност и API токени](/guides/webhooks/webhooks-api-tokens) за информация относно проверката на HMAC подписа.

---