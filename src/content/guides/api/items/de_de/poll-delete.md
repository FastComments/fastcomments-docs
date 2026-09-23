[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Entfernt eine Umfrage aus ihrem Kommentar, zusammen mit allen darauf abgegebenen Stimmen. Der Kommentar selbst bleibt unverändert.

Das Löschen des Kommentars entfernt ebenfalls seine Umfrage und Stimmen, daher wird dies nur benötigt, wenn Sie den Kommentar behalten möchten.

[inline-code-attrs-start title = 'Umfrage Löschen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Umfrage Löschen Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Umfrage Löschen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** Bei Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Bei Fehler enthalten. **/
    reason?: string
}
[inline-code-end]