[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Diese Route bietet die Möglichkeit, einen Login-Link an einen einzelnen `TenantUser` zu senden.

Nützlich beim Batch-Erstellen von Benutzern, ohne ihnen erklären zu müssen, wie sie sich bei FastComments.com anmelden. Dies sendet ihnen einfach einen "Magic Link" zum Anmelden, der
nach `30 Tagen` abläuft.

Die folgenden Einschränkungen gelten für das Senden eines Login-Links an einen `TenantUser`:
- Der `TenantUser` muss bereits existieren.
- Sie müssen Zugriff haben, um den `Tenant` zu verwalten, zu dem der `TenantUser` gehört.

Wir können wie folgt einen Login-Link an einen `TenantUser` senden:

[inline-code-attrs-start title = 'TenantUser Login-Link cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Dies sendet eine E-Mail wie `Bob bei TenantName lädt Sie ein, Moderator zu werden...`

[inline-code-attrs-start title = 'TenantUser Login-Link Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Login-Link Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
