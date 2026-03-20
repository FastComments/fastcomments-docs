Jedynej struktury wysyłanej przez webhooks jest obiekt WebhookComment, opisany poniżej w TypeScript.

#### Struktura obiektu WebhookComment

##### Struktura zdarzenia "create"
Ciało żądania zdarzenia "create" to obiekt WebhookComment.

##### Struktura zdarzenia "update"
Ciało żądania zdarzenia "update" to obiekt WebhookComment.

##### Struktura zdarzenia "delete"
Ciało żądania zdarzenia "delete" to obiekt WebhookComment.

    Zmiana od 14 listopada 2023
    Wcześniej ciało żądania zdarzenia "delete" zawierało tylko id komentarza. Teraz zawiera pełny komentarz w momencie usunięcia.


[inline-code-attrs-start title = 'Obiekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id komentarza. **/
    id: string
    /** Id lub URL identyfikujący wątek komentarzy. Znormalizowany. **/
    urlId: string
    /** URL wskazujący miejsce, gdzie został dodany komentarz. **/
    url?: string
    /** Id użytkownika, który dodał komentarz. W przypadku SSO, poprzedzone tenant id. **/
    userId?: string
    /** Email użytkownika, który dodał komentarz. **/
    commenterEmail?: string
    /** Nazwa użytkownika wyświetlana w widżecie komentarza. W przypadku SSO, może być displayName. **/
    commenterName: string
    /** Surowy tekst komentarza. **/
    comment: string
    /** Tekst komentarza po parsowaniu. **/
    commentHTML: string
    /** Zewnętrzne id komentarza. **/
    externalId?: string
    /** Id komentarza nadrzędnego. **/
    parentId?: string | null
    /** Data UTC, kiedy komentarz został dodany. **/
    date: UTC_ISO_DateString
    /** Suma karmy (up - down) z głosów. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Prawda jeśli użytkownik był zalogowany podczas dodawania komentarza, lub jeśli zweryfikował komentarz, lub jeśli zweryfikował swoją sesję w momencie dodania komentarza. **/
    verified: boolean
    /** Data weryfikacji komentarza. **/
    verifiedDate?: number
    /** Czy moderator oznaczył komentarz jako sprawdzony. **/
    reviewed: boolean
    /** Lokacja lub zakodowany base64 awatara. Będzie base64 tylko jeśli taka była wartość przekazana przy SSO. **/
    avatarSrc?: string
    /** Czy komentarz został oznaczony jako spam ręcznie czy automatycznie? **/
    isSpam: boolean
    /** Czy komentarz został automatycznie oznaczony jako spam? **/
    aiDeterminedSpam: boolean
    /** Czy w komentarzu znajdują się obrazy? **/
    hasImages: boolean
    /** Numer strony, na której znajduje się komentarz przy sortowaniu "Most Relevant". **/
    pageNumber: number
    /** Numer strony przy sortowaniu "Oldest First". **/
    pageNumberOF: number
    /** Numer strony przy sortowaniu "Newest First". **/
    pageNumberNF: number
    /** Czy komentarz został zatwierdzony automatycznie czy ręcznie? **/
    approved: boolean
    /** Kod lokalizacji (format: en_us) użytkownika w momencie dodawania komentarza. **/
    locale: string
    /** Wzmianki @ zapisane w komentarzu, które zostały pomyślnie sparsowane. **/
    mentions?: CommentUserMention[]
    /** Domena, z której pochodzi komentarz. **/
    domain?: string
    /** Opcjonalna lista identyfikatorów (id) grup moderacyjnych powiązanych z tym komentarzem. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Gdy użytkownicy są oznaczani w komentarzu, informacja jest przechowywana na liście o nazwie `mentions`. Każdy obiekt na tej liście ma następującą strukturę.

[inline-code-attrs-start title = 'Obiekt wzmianki Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id użytkownika. W przypadku użytkowników SSO będzie poprzedzone tenant id. **/
    id: string
    /** Końcowy tekst tagu @mention, łącznie z symbolem @. **/
    tag: string
    /** Oryginalny tekst tagu @mention, łącznie z symbolem @. **/
    rawTag: string
    /** Jaki typ użytkownika został oznaczony. user = konto FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Jeśli użytkownik zrezygnuje z powiadomień, to mimo to będzie ustawione na true. **/
    sent: boolean
}
[inline-code-end]

#### Metody HTTP

Możesz skonfigurować metodę HTTP dla każdego typu zdarzenia webhook w panelu administracyjnym:

- **Create Event**: POST lub PUT (domyślnie: PUT)
- **Update Event**: POST lub PUT (domyślnie: PUT)
- **Delete Event**: DELETE, POST lub PUT (domyślnie: DELETE)

Ponieważ wszystkie żądania zawierają ID, operacje Create i Update są domyślnie idempotentne (PUT). Powtarzanie tego samego żądania Create lub Update nie powinno tworzyć duplikatów po Twojej stronie.

#### Nagłówki żądań

Każde żądanie webhook zawiera następujące nagłówki:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Twój sekret API |
| `X-FastComments-Timestamp` | Znacznik czasu Unix (sekundy) w momencie podpisania żądania |
| `X-FastComments-Signature` | Podpis HMAC-SHA256 (`sha256=<hex>`) |

Zobacz [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) aby uzyskać informacje o weryfikacji podpisu HMAC.

---