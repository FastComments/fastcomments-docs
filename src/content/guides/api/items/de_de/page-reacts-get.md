[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Gibt die Anzahl für jede Reaktion auf einer Seite zurück und welche Reaktionen der aktuelle Benutzer hinzugefügt hat.

[inline-code-attrs-start title = 'Page Reacts cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI-codiertes JSON Ihres SSO-Objekts. Für anonyme Benutzer weglassen. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: string
    /** Bei einem Fehler enthalten. **/
    reason?: string
    /** Anzahl pro Reaktions-ID, zum Beispiel {"heart": 12, "laugh": 3}. Nicht gesetzt, wenn die Seite keine Reaktionen hat. **/
    counts?: Record<string, number>
    /** Die Reaktions-IDs, die der Benutzer, der die Anfrage stellt, hinzugefügt hat. Nicht gesetzt, wenn er keine hinzugefügt hat. **/
    reactedIds?: string[]
}
[inline-code-end]