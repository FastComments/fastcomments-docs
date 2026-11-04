Jedyną strukturą wysyłaną przez webhooki jest obiekt WebhookComment, przedstawiony poniżej w TypeScript.

#### Struktura obiektu WebhookComment

##### Struktura zdarzenia "Create"
Treść żądania zdarzenia "create" jest obiektem WebhookComment.

##### Struktura zdarzenia "Update"
Treść żądania zdarzenia "update" jest obiektem WebhookComment.

##### Struktura zdarzenia "Delete"
Treść żądania zdarzenia "delete" jest obiektem WebhookComment.

    Zmiana z 14 listopada 2023
    Wcześniej treść żądania zdarzenia "delete" zawierała tylko identyfikator komentarza. Teraz zawiera pełny komentarz w momencie usunięcia.


Każdy klucz jest zawsze obecny w treści. Gdy komentarz nie ma wartości dla danego pola, treść zawiera `null`
(lub `false` dla wartości logicznych oraz `[]` dla list), dzięki czemu kształt dostarczanych danych nigdy nie różni się między komentarzami.

[inline-code-attrs-start title = 'Obiekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Gdy użytkownicy są oznaczani w komentarzu, informacje są przechowywane na liście o nazwie `mentions`. Każdy obiekt na tej liście
ma następującą strukturę.

[inline-code-attrs-start title = 'Obiekt Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

Każdy wpis na liście `badges` ma następującą strukturę. Zobacz [przewodnik po odznakach](/guide-badges.html), aby poznać typy odznak.

[inline-code-attrs-start title = 'Obiekt Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### Metody HTTP

Metodę HTTP dla każdego typu zdarzenia webhooka można skonfigurować w panelu administracyjnym:

- **Zdarzenie Create**: POST lub PUT (domyślnie: PUT)
- **Zdarzenie Update**: POST lub PUT (domyślnie: PUT)
- **Zdarzenie Delete**: DELETE, POST lub PUT (domyślnie: DELETE)

Ponieważ wszystkie żądania zawierają identyfikator, operacje Create i Update są domyślnie idempotentne (PUT). Powtórzenie tego samego żądania Create lub Update nie powinno tworzyć duplikatów obiektów po Twojej stronie.

#### Nagłówki żądania

Każde żądanie webhooka zawiera następujące nagłówki:

| Nagłówek | Opis |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Twój sekret API |
| `X-FastComments-Timestamp` | Znacznik czasu Unix (sekundy) w momencie podpisania żądania |
| `X-FastComments-Signature` | Podpis HMAC-SHA256 (`sha256=<hex>`) |

Zobacz [Bezpieczeństwo i tokeny API](/guide-webhooks.html#webhooks-api-tokens), aby uzyskać informacje na temat weryfikacji podpisu HMAC.