[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Zwraca liczbę polubień na stronie oraz informację, czy bieżący użytkownik ją polubił. Strony, które jeszcze nie istnieją, zwracają `likeCount` równy `0`.

Przekaż `includeUserInfo=public`, aby również otrzymać użytkowników, którzy polubili stronę, w kolejności od najnowszych. Użytkownicy z prywatną aktywnością profilu oraz anonimowi użytkownicy są pomijani, a przeszukiwane jest maksymalnie 100 polubień. Tylko `public` jest akceptowane; `include-private` zwraca `422` i jest dostępne pod [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) przy użyciu klucza API. Na stronach wymagających SSO, `includeUserInfo` zwraca `403` dla użytkowników, którzy nie mogą wyświetlić strony.

[inline-code-attrs-start title = 'Przykład cURL dla polubień strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania polubień strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** Zakodowany w URI JSON Twojego obiektu SSO. Pomiń dla anonimowych użytkowników. **/
    sso?: string
    /** Ustaw na 'public', aby uwzględnić użytkowników, którzy polubili stronę. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi polubień strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    likeCount: number
    /** Czy użytkownik wykonujący żądanie polubił stronę. **/
    didLike: boolean
    /** Liczba komentarzy najwyższego poziomu na stronie. **/
    commentCount: number
    /** Identyfikator używany do subskrypcji aktualizacji na żywo dla tej strony. **/
    urlIdWS: string
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

---