[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Entfernt eine der Reaktionen des aktuellen Benutzers von einer Seite. Wenn der Benutzer diese Reaktion nicht hinzugefügt hat, ist die Anfrage erfolgreich mit dem Code `no-react` und ändert die Anzahl nicht.

[inline-code-attrs-start title = 'Page React Delete cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Page React Delete Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Die Reaktions-ID. **/
    id: string
    /** URI-codiertes JSON Ihres SSO-Objekts. Für anonyme Benutzer weglassen. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page React Delete Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react', wenn der Benutzer diese Reaktion nicht hinzugefügt hat. Andernfalls bei einem Fehler enthalten. **/
    code?: 'no-react' | string
    /** Bei einem Fehler enthalten. **/
    reason?: string
}
[inline-code-end]

---