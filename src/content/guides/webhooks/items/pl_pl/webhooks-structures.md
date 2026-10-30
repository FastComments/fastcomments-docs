The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Struktura obiektu WebhookComment

##### Struktura zdarzenia „Create”
The "create" event request body is a WebhookComment object.

##### Struktura zdarzenia „Update”
The "update" event request body is a WebhookComment object.

##### Struktura zdarzenia „Delete”
The "delete" event request body is a WebhookComment object.

    Zmiana od 14 listopada 2023
    Wcześniej ciało żądania zdarzenia „delete” zawierało tylko identyfikator komentarza. Teraz zawiera pełny komentarz w momencie usunięcia.

Każdy klucz jest zawsze obecny w ciele. Gdy komentarz nie ma wartości dla pola, ciało zawiera `null` (lub `false` dla wartości logicznych i `[]` dla list), więc struktura dostawy nigdy nie różni się między kolejnymi komentarzami.

[inline-code-attrs-start title = 'Obiekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Identyfikator komentarza. **/
    id: string
    /** Identyfikator lub URL identyfikujący wątek komentarza. Znormalizowany. **/
    urlId: string
    /** URL wskazujący miejsce, w którym pozostano komentarz. **/
    url: string | null
    /** Identyfikator użytkownika, który pozostawił komentarz. W przypadku SSO poprzedzony identyfikatorem najemcy. **/
    userId: string | null
    /** Adres e‑mail użytkownika, który pozostawił komentarz. **/
    commenterEmail: string | null
    /** Nazwa użytkownika wyświetlana w widżecie komentarza. W przypadku SSO może być displayName. **/
    commenterName: string
    /** Surowy tekst komentarza. **/
    comment: string
    /** Tekst komentarza po przetworzeniu. **/
    commentHTML: string
    /** Zewnętrzny identyfikator komentarza. **/
    externalId: string | null
    /** Identyfikator komentarza nadrzędnego. **/
    parentId: string | null
    /** Data w formacie UTC, kiedy komentarz został pozostany. **/
    date: UTC_ISO_DateString
    /** Łączna karma (głosy w górę - w dół). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Prawda, jeśli użytkownik był zalogowany podczas komentowania, zweryfikował komentarz lub zweryfikował sesję w momencie pozostawienia komentarza. **/
    verified: boolean
    /** Data w formacie UTC, kiedy komentarz został zweryfikowany. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Jeśli moderator oznaczył komentarz jako sprawdzony. **/
    reviewed: boolean
    /** Lokalizacja lub kodowanie base64 awatara. Będzie base64 tylko wtedy, gdy taka wartość została przekazana z SSO. **/
    avatarSrc: string | null
    /** Czy komentarz został oznaczony jako spam ręcznie lub automatycznie? **/
    isSpam: boolean
    /** Czy komentarz został automatycznie oznaczony jako spam? **/
    aiDeterminedSpam: boolean
    /** Czy w komentarzu znajdują się obrazy? **/
    hasImages: boolean
    /** Numer strony, na której znajduje się komentarz w sortowaniu „Most Relevant”. **/
    pageNumber: number | null
    /** Numer strony, na której znajduje się komentarz w sortowaniu „Oldest First”. **/
    pageNumberOF: number | null
    /** Numer strony, na której znajduje się komentarz w sortowaniu „Newest First”. **/
    pageNumberNF: number | null
    /** Czy komentarz został zatwierdzony automatycznie lub ręcznie? **/
    approved: boolean
    /** Kod lokalizacji (format: en_us) użytkownika w momencie napisania komentarza. **/
    locale: string | null
    /** Wzmianki @ zapisane w komentarzu, które zostały pomyślnie przetworzone. Puste, gdy ich brak. **/
    mentions: CommentUserMention[]
    /** Domena, z której pochodzi komentarz. **/
    domain: string | null
    /** Identyfikatory grup moderacji powiązane z tym komentarzem. Puste, gdy ich brak. **/
    moderationGroupIds: string[]
    /** Odznaki wyświetlane obok nazwy komentującego w tym komentarzu, w momencie utworzenia zdarzenia. Puste, gdy ich brak. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Gdy użytkownicy są oznaczani w komentarzu, informacje są przechowywane na liście o nazwie `mentions`. Każdy obiekt na tej liście ma następującą strukturę.

[inline-code-attrs-start title = 'Obiekt Wzmianki Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Identyfikator użytkownika. Dla użytkowników SSO będzie poprzedzony identyfikatorem najemcy. **/
    id: string
    /** Końcowy tekst tagu @mention, włącznie z symbolem @. **/
    tag: string
    /** Oryginalny tekst tagu @mention, włącznie z symbolem @. **/
    rawTag: string
    /** Typ oznaczonego użytkownika. user = konto FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Jeśli użytkownik rezygnuje z powiadomień, to pole nadal będzie ustawione na true. **/
    sent: boolean
}
[inline-code-end]

Każdy wpis na liście `badges` ma następującą strukturę. Zobacz [przewodnik po odznakach](/guide-badges.html) aby poznać typy odznak.

[inline-code-attrs-start title = 'Obiekt Odznaki Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** Identyfikator odznaki przyznanej temu użytkownikowi. **/
    id: string
    /** Typ odznaki. **/
    type: number
    /** Opis odznaki, wyświetlany po najechaniu na odznakę. **/
    description: string
    /** Tekst wyświetlany na odznace. **/
    displayLabel?: string | null
    /** Obraz wyświetlany dla odznaki, jeśli jest używany. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Dodatkowa klasa CSS zastosowana do odznaki. **/
    cssClass?: string | null
}
[inline-code-end]

#### Metody HTTP

Możesz skonfigurować metodę HTTP dla każdego typu zdarzenia webhook w panelu administracyjnym:

- **Zdarzenie Create**: POST lub PUT (domyślnie: PUT)
- **Zdarzenie Update**: POST lub PUT (domyślnie: PUT)
- **Zdarzenie Delete**: DELETE, POST lub PUT (domyślnie: DELETE)

Ponieważ wszystkie żądania zawierają identyfikator, operacje Create i Update są domyślnie idempotentne (PUT). Powtarzanie tego samego żądania Create lub Update nie powinno tworzyć duplikatów obiektów po Twojej stronie.

#### Nagłówki żądania

Każde żądanie webhook zawiera następujące nagłówki:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Twój sekret API |
| `X-FastComments-Timestamp` | Znacznik czasu Unix (sekundy) w momencie podpisania żądania |
| `X-FastComments-Signature` | Podpis HMAC-SHA256 (`sha256=<hex>`) |

Zobacz [Bezpieczeństwo i tokeny API](/guide-webhooks.html#webhooks-api-tokens) aby uzyskać informacje o weryfikacji podpisu HMAC.

---