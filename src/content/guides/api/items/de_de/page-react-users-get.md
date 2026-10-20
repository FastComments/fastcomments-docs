[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Gibt die Namen der Benutzer zurück, die einer Seite eine Reaktion hinzugefügt haben, alphabetisch sortiert. Es werden bis zu 100 Reaktionen abgefragt, und anonyme Benutzer sowie Benutzer mit privater Profilaktivität werden nicht einbezogen. Auf Seiten, die SSO erfordern, erhalten Benutzer, die die Seite nicht anzeigen können, einen `403` mit dem Code `sso-required` oder `page-forbidden`.

Um auch Avatare und Benutzer‑IDs zu erhalten, verwenden Sie [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) mit `includeUserInfo=public`.

[inline-code-attrs-start title = 'Page React Users cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]