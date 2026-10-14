[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Zwraca liczbę dla każdej reakcji na stronie, a opcjonalnie także użytkowników, którzy zareagowali, jako jeden wpis na reakcję z jej identyfikatorem, najnowsze najpierw. To jest wersja po stronie serwera [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` kontroluje, którzy użytkownicy są zwracani:

- `public` pomija użytkowników, którzy ustawili swoją aktywność profilu jako prywatną.
- `include-private` włącza ich. Używaj tego tylko wtedy, gdy lista nie jest wyświetlana publicznie.

Użytkownicy anonimowi nigdy nie są uwzględniani, a przeszukiwane jest maksymalnie 100 reakcji. Strony, które nie istnieją, nie zwracają `counts` i zwracają pustą listę `users`.

[inline-code-attrs-start title = 'Przykład cURL dla Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the counts. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Wskazówka

Pamiętaj, aby kodować wartości URI, takie jak `urlId`.