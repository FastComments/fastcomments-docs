[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Zwraca liczbę polubień na stronie oraz opcjonalnie użytkowników, którzy ją polubili, najpierw najnowsze. Jest to wersja po stronie serwera [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` kontroluje, którzy użytkownicy są zwracani:

- `public` pomija użytkowników, którzy ustawili swoją aktywność profilu jako prywatną.
- `include-private` uwzględnia ich. Używaj tego tylko wtedy, gdy lista nie jest wyświetlana publicznie.

Użytkownicy anonimowi nigdy nie są uwzględniani, a przeszukiwane jest maksymalnie 100 polubień. Strony, które nie istnieją, zwracają `likeCount` równy `0` oraz pustą listę `users` list.

[inline-code-attrs-start title = 'Przykład cURL Lajków Strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Żądania Lajków Strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Pomiń, aby otrzymać tylko liczbę polubień. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Odpowiedzi Lajków Strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    likeCount?: number
    /** Najpierw najnowsze. Ustawiane tylko, gdy przekazano includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Przydatna wskazówka

Pamiętaj, aby kodować wartości URI, takie jak `urlId`.