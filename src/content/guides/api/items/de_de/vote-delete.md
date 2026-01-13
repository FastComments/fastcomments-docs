[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Diese Route bietet die Möglichkeit, ein einzelnes `Vote` zu löschen.

[inline-code-attrs-start title = 'Vote Löschen cURL Beispiel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote Löschen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote Löschen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

Hinweise:

- Diese API befolgt Einstellungen auf Tenant-Ebene. Wenn Sie beispielsweise das Abstimmen für eine bestimmte Seite deaktivieren und versuchen, über die API eine Abstimmung zu erstellen, schlägt dies mit dem Fehlercode `voting-disabled` fehl.
- Diese API ist standardmäßig live.
- Diese API aktualisiert die `votes` des entsprechenden `Comment`.
