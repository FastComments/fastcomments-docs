Jedyną strukturą wysyłaną przez webhooki jest obiekt WebhookComment, przedstawiony poniżej w TypeScript.

#### Struktura obiektu WebhookComment

##### Struktura zdarzenia "create"
Ciało żądania zdarzenia "create" jest obiektem WebhookComment.

##### Struktura zdarzenia "update"
Ciało żądania zdarzenia "update" jest obiektem WebhookComment.

##### Struktura zdarzenia "delete"
Ciało żądania zdarzenia "delete" jest obiektem WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Obiekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id komentarza. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** URL wskazujący miejsce, gdzie zamieszczono komentarz. **/
    url?: string
    /** Id użytkownika, który dodał komentarz. Jeśli SSO, poprzedzony tenant id. **/
    userId?: string
    /** Email użytkownika, który dodał komentarz. **/
    commenterEmail?: string
    /** Nazwa użytkownika wyświetlana w widżecie komentarzy. Przy SSO może być to displayName. **/
    commenterName: string
    /** Surowy tekst komentarza. **/
    comment: string
    /** Tekst komentarza po przetworzeniu. **/
    commentHTML: string
    /** Zewnętrzny identyfikator komentarza. **/
    externalId?: string
    /** Id nadrzędnego komentarza. **/
    parentId?: string | null
    /** Data UTC, kiedy komentarz został dodany. **/
    date: UTC_ISO_DateString
    /** Suma punktów (głosy za - głosy przeciw). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Prawda, jeśli użytkownik był zalogowany podczas dodawania komentarza, jeśli komentarz został zweryfikowany przez użytkownika, lub jeśli użytkownik zweryfikował sesję w momencie dodania komentarza. **/
    verified: boolean
    /** Data, kiedy komentarz został zweryfikowany. **/
    verifiedDate?: number
    /** Czy moderator oznaczył komentarz jako przejrzany. **/
    reviewed: boolean
    /** Lokalizacja lub kodowanie base64 avatara. Będzie base64 tylko jeśli taka wartość została przekazana wraz z SSO. **/
    avatarSrc?: string
    /** Czy komentarz został oznaczony jako spam ręcznie czy automatycznie? **/
    isSpam: boolean
    /** Czy komentarz został automatycznie oznaczony jako spam? **/
    aiDeterminedSpam: boolean
    /** Czy w komentarzu znajdują się obrazy? **/
    hasImages: boolean
    /** Numer strony, na której znajduje się komentarz przy sortowaniu "Most Relevant". **/
    pageNumber: number
    /** Numer strony, na której znajduje się komentarz przy sortowaniu "Oldest First". **/
    pageNumberOF: number
    /** Numer strony, na której znajduje się komentarz przy sortowaniu "Newest First". **/
    pageNumberNF: number
    /** Czy komentarz został zatwierdzony automatycznie czy ręcznie? **/
    approved: boolean
    /** Kod lokalizacji (format: en_us) użytkownika w momencie napisania komentarza. **/
    locale: string
    /** The @mentions written in the comment that were successfully parsed. **/
    mentions?: CommentUserMention[]
    /** Domeną, z której pochodzi komentarz. **/
    domain?: string
    /** Opcjonalna lista identyfikatorów grup moderacyjnych powiązanych z tym komentarzem. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Obiekt wzmianki Webhook'; type = 'typescript'; inline-code-attrs-end]
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

#### Używane metody HTTP

**Create i Update używają HTTP PUT, a nie POST!**

Ponieważ wszystkie nasze żądania zawierają identyfikator, powtórzenie tego samego żądania Create lub Update nie powinno tworzyć nowych obiektów po Twojej stronie.

Oznacza to, że te wywołania są idempotentne i zgodnie ze specyfikacją HTTP powinny być traktowane jako zdarzenia PUT.

---