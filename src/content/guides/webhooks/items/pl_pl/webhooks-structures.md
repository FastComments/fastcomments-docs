Jedyną strukturą wysyłaną przez webhooki jest obiekt WebhookComment, przedstawiony poniżej w TypeScript.

#### Struktura obiektu WebhookComment

##### Struktura zdarzenia "create"
Ciało żądania zdarzenia "create" jest obiektem WebhookComment.

##### Struktura zdarzenia "update"
Ciało żądania zdarzenia "update" jest obiektem WebhookComment.

##### Struktura zdarzenia "delete"
Ciało żądania zdarzenia "delete" jest obiektem WebhookComment.

    Zmiana od 14 listopada 2023
    Wcześniej ciało żądania zdarzenia "delete" zawierało tylko identyfikator komentarza. Teraz zawiera pełny komentarz w momencie usunięcia.


[inline-code-attrs-start title = 'Obiekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id komentarza. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** URL wskazujący miejsce, gdzie został dodany komentarz. **/
    url?: string
    /** Id użytkownika, który dodał komentarz. Jeśli SSO, poprzedzone tenant id. **/
    userId?: string
    /** Email użytkownika, który dodał komentarz. **/
    commenterEmail?: string
    /** Nazwa użytkownika wyświetlana w widżecie komentarzy. Przy SSO może to być displayName. **/
    commenterName: string
    /** Surowy tekst komentarza. **/
    comment: string
    /** Tekst komentarza po parsowaniu. **/
    commentHTML: string
    /** Zewnętrzny id komentarza. **/
    externalId?: string
    /** Id komentarza nadrzędnego. **/
    parentId?: string | null
    /** Data w UTC, kiedy komentarz został dodany. **/
    date: UTC_ISO_DateString
    /** Suma głosów (up - down). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Prawda, jeśli użytkownik był zalogowany podczas dodawania komentarza, lub zweryfikował komentarz, albo zweryfikował swoją sesję w momencie dodania komentarza. **/
    verified: boolean
    /** Data, kiedy komentarz został zweryfikowany. **/
    verifiedDate?: number
    /** Czy moderator oznaczył komentarz jako sprawdzony. **/
    reviewed: boolean
    /** Lokalizacja lub kodowanie base64 awatara. Będzie base64 tylko wtedy, gdy taka wartość została przekazana z SSO. **/
    avatarSrc?: string
    /** Czy komentarz został oznaczony jako spam ręcznie czy automatycznie? **/
    isSpam: boolean
    /** Czy komentarz został automatycznie oznaczony jako spam? **/
    aiDeterminedSpam: boolean
    /** Czy w komentarzu są obrazy? **/
    hasImages: boolean
    /** Numer strony, na której znajduje się komentarz dla kierunku sortowania "Most Relevant". **/
    pageNumber: number
    /** Numer strony, na której znajduje się komentarz dla kierunku sortowania "Oldest First". **/
    pageNumberOF: number
    /** Numer strony, na której znajduje się komentarz dla kierunku sortowania "Newest First". **/
    pageNumberNF: number
    /** Czy komentarz został zatwierdzony automatycznie czy ręcznie? **/
    approved: boolean
    /** Kod lokalizacji (format: en_us) użytkownika w momencie napisania komentarza. **/
    locale: string
    /** @mentions napisane w komentarzu, które zostały pomyślnie sparsowane. **/
    mentions?: CommentUserMention[]
    /** Domena, z której pochodzi komentarz. **/
    domain?: string
    /** Opcjonalna lista id grup moderacji powiązanych z tym komentarzem. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Gdy użytkownicy są oznaczani w komentarzu, informacje są przechowywane w liście o nazwie `mentions`. Każdy obiekt w tej liście ma następującą strukturę.

[inline-code-attrs-start title = 'Obiekt wzmianki webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id użytkownika. Dla użytkowników SSO będzie poprzedzony tenant id. **/
    id: string
    /** Ostateczny tekst tagu @mention, łącznie z symbolem @. **/
    tag: string
    /** Oryginalny tekst tagu @mention, łącznie z symbolem @. **/
    rawTag: string
    /** Jaki typ użytkownika został oznaczony. user = konto FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Jeśli użytkownik zrezygnuje z powiadomień, ta wartość nadal będzie ustawiona na true. **/
    sent: boolean
}
[inline-code-end]

#### Metody HTTP

Możesz skonfigurować metodę HTTP dla każdego typu zdarzenia webhook w panelu administracyjnym:

- **Create Event**: POST lub PUT (domyślnie: PUT)
- **Update Event**: POST lub PUT (domyślnie: PUT)
- **Delete Event**: DELETE, POST lub PUT (domyślnie: DELETE)

Ponieważ wszystkie żądania zawierają ID, operacje Create i Update są domyślnie idempotentne (PUT). Powtórzenie tego samego żądania Create lub Update nie powinno tworzyć zduplikowanych obiektów po Twojej stronie.

#### Nagłówki żądań

Każde żądanie webhook zawiera następujące nagłówki:

| Nagłówek | Opis |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Twój sekret API |
| `X-FastComments-Timestamp` | Znacznik czasu Unix (sekundy) w momencie podpisania żądania |
| `X-FastComments-Signature` | Podpis HMAC-SHA256 (`sha256=<hex>`) |

Zobacz [Security & API Tokens](/guides/webhooks/webhooks-api-tokens) aby uzyskać informacje o weryfikacji podpisu HMAC.

---