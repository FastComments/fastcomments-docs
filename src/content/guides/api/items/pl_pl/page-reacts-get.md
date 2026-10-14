[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Zwraca liczbę dla każdej reakcji na stronie oraz które reakcje dodał bieżący użytkownik.

Przekaż `includeUserInfo=public`, aby również uzyskać użytkowników, którzy zareagowali, jako jeden wpis na reakcję z jej identyfikatorem, najnowsze najpierw. Użytkownicy z prywatną aktywnością profilu oraz anonimowi użytkownicy są pomijani, a przeszukiwane jest maksymalnie 100 reakcji. Tylko `public` jest tutaj akceptowane; `include-private` zwraca `422` i jest dostępne z [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) przy użyciu klucza API. Na stronach wymagających SSO, `includeUserInfo` zwraca `403` dla użytkowników, którzy nie mogą wyświetlić strony.

[inline-code-attrs-start title = 'Przykład cURL dla Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** Zakodowany w URI JSON twojego obiektu SSO. Pomiń dla anonimowych użytkowników. **/
    sso?: string
    /** Ustaw na 'public', aby uwzględnić użytkowników, którzy zareagowali. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Liczba dla każdego identyfikatora reakcji, np. {"heart": 12, "laugh": 3}. Nie ustawiane, gdy strona nie ma reakcji. **/
    counts?: Record<string, number>
    /** Identyfikatory reakcji, które dodał użytkownik wykonujący żądanie. Nie ustawiane, gdy nie dodał żadnych. **/
    reactedIds?: string[]
    /** Jeden wpis na reakcję, najnowsze najpierw. Ustawiane tylko, gdy przekazano includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Identyfikator reakcji, np. "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]