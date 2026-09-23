[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Zwraca liczbę polubień na stronie oraz informację, czy bieżący użytkownik ją polubił. Strony, które jeszcze nie istnieją, zwracają `likeCount` równy `0`.

[inline-code-attrs-start title = 'Przykład cURL dla polubień strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania polubień strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi polubień strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
}
[inline-code-end]

---