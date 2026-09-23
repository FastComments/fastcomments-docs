[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Entfernt das Like des aktuellen Benutzers von einer Seite. Wenn der Benutzer die Seite nicht geliked hat, schlägt die Anfrage mit dem Code `not-liked` erfolgreich ab und ändert die Anzahl nicht.

[inline-code-attrs-start title = 'Seiten-Unlike cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Seiten-Unlike Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Seiten-Unlike Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' when the user had not liked the page. Otherwise included on failure. **/
    code?: 'not-liked' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]