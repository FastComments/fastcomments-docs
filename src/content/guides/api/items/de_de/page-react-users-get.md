[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Gibt die Namen der Benutzer zurück, die einer Seite eine Reaktion hinzugefügt haben, alphabetisch sortiert. Es werden bis zu 100 Reaktionen abgefragt, und anonyme Benutzer werden nicht einbezogen.

[inline-code-attrs-start title = 'Page React Users cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Die Reaktions-ID. **/
    id: string
    /** URI-codiertes JSON Ihres SSO-Objekts. Für anonyme Benutzer weglassen. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: string
    /** Bei einem Fehler enthalten. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]