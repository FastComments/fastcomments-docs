Единствената структура, изпращана чрез webhooks, е обектът WebhookComment, описан по-долу в TypeScript.

#### The WebhookComment Object Structure

##### Структура на събитието "Create"
Тялото на заявката за събитието "create" е обект WebhookComment.

##### Структура на събитието "Update"
Тялото на заявката за събитието "update" е обект WebhookComment.

##### Структура на събитието "Delete"
Тялото на заявката за събитието "delete" е обект WebhookComment.

    Промяна от Nov 14th 2023
    Преди тялото на заявката за събитието "delete" съдържаше само id на коментара. Сега то съдържа целия коментар към момента на изтриване.


[inline-code-attrs-start title = 'Обектът WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Идентификаторът на коментара. **/
    id: string
    /** Идентификаторът или URL, който идентифицира нишката с коментари. Нормализиран. **/
    urlId: string
    /** URL, сочещ към мястото, където е оставен коментарът. **/
    url?: string
    /** Идентификаторът на потребителя, който е оставил коментара. Ако е SSO, е с предварен префикс tenant id. **/
    userId?: string
    /** Имейлът на потребителя, който е оставил коментара. **/
    commenterEmail?: string
    /** Името на потребителя, което се показва в коментарния уиджет. При SSO може да бъде displayName. **/
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
    /** Комбинирана карма (up - down) от гласовете. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ако потребителят е бил влязъл когато е коментирал, или ако е верифицирал коментара, или ако е верифицирал сесията си когато коментарът е бил оставен. **/
    verified: boolean
    /** Дата, когато коментарът е бил верифициран. **/
    verifiedDate?: number
    /** Ако модератор е отбелязал коментара като прегледан. **/
    reviewed: boolean
    /** Местоположението или base64-кодираното изображение на аватара. Ще бъде base64 само ако това е стойността, подадена със SSO. **/
    avatarSrc?: string
    /** Беше ли коментарът маркиран като спам ръчно или автоматично? **/
    isSpam: boolean
    /** Беше ли коментарът автоматично маркиран като спам? **/
    aiDeterminedSpam: boolean
    /** Има ли изображения в коментара? **/
    hasImages: boolean
    /** Номерът на страницата, на която се намира коментарът за посоката на сортиране "Most Relevant". **/
    pageNumber: number
    /** Номерът на страницата за сортиране "Oldest First". **/
    pageNumberOF: number
    /** Номерът на страницата за сортиране "Newest First". **/
    pageNumberNF: number
    /** Коментарът одобрен ли е автоматично или ръчно? **/
    approved: boolean
    /** Локалният код (формат: en_us) на потребителя, когато е написан коментарът. **/
    locale: string
    /** @споменаванията, написани в коментара, които бяха успешно парснати. **/
    mentions?: CommentUserMention[]
    /** Домейнът, от който е коментарът. **/
    domain?: string
    /** По избор списък с идентификатори на модерационни групи, свързани с този коментар. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Обектът за споменавания в Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Идентификаторът на потребителя. За SSO потребители ще има предварен префикс tenant id. **/
    id: string
    /** Крайният текст на @mention тагa, включително символа @. **/
    tag: string
    /** Оригиналният текст на @mention тага, включително символа @. **/
    rawTag: string
    /** Какъв тип потребител е бил тагнат. user = FastComments.com акаунт. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ако потребителят се откаже от известия, това все пак ще бъде зададено на true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods Used

**Create and Update both use HTTP PUT and not POST!**

Since all of our requests contain an ID, repeating the same Create or Update request should not create new objects on your side.

This means that these calls are idempotent and should be PUT events as per the HTTP specification.

---