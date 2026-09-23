[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Polubić stronę jako bieżący użytkownik. Każdy użytkownik może polubić stronę raz: ponowne polubienie kończy się kodem `already-liked` i nie zmienia licznika.

Strona zostanie utworzona, jeśli jeszcze nie istnieje. Przekaż `title`, aby ustawić lub zaktualizować tytuł strony.

[inline-code-attrs-start title = 'Przykład cURL polubienia strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania polubienia strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi polubienia strony'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' when the user had already liked the page. Otherwise included on failure. **/
    code?: 'already-liked' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]