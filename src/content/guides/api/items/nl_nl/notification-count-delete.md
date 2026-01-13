[api-resource-header-start name = 'NotificationCount'; route = 'DELETE /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Deze route verwijdert een enkele `NotificationCount` op basis van gebruikers-id. Bij SSO heeft de gebruikers-id het formaat `<tenant id>:<user id>`.

Dit wist de ongelezen meldingtelling van de gebruiker (het rode bel-icoon in de reactie-widget vervaagt en het aantal verdwijnt).

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor DELETE NotificationCount'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success"}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van verwijderingsverzoek voor NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van antwoord op verwijdering van NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]