[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Usuwa polubienie bieżącego użytkownika ze strony. Jeśli użytkownik nie polubił strony, żądanie kończy się sukcesem z kodem `not-liked` i nie zmienia licznika.

[inline-code-attrs-start title = 'Przykład cURL usunięcia polubienia strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania usunięcia polubienia strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usunięcia polubienia strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' when the user had not liked the page. Otherwise included on failure. **/
    code?: 'not-liked' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]