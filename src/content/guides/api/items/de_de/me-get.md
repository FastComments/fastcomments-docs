[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Beschreibt das Anmeldeinformation, das die Anfrage stellt: den Mandanten, zu dem es gehört, und bei OAuth‑Zugriffstoken den Benutzer, der die Anwendung autorisiert hat. Integrationen verwenden es, um eine Verbindung zu testen und zu kennzeichnen.

Bei einem API‑Schlüssel identifiziert die Antwort nur den Mandanten. Bei einem OAuth‑Bearer‑Token enthält sie außerdem den autorisierenden Benutzer und die gewährten Berechtigungen.

[inline-code-attrs-start title = 'Me cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** How the request was authenticated. **/
    authType: 'api-key' | 'oauth'
    /** The scopes the credential holds. API keys hold both. **/
    scopes: ('read' | 'write')[]
    /** Only present for OAuth tokens. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]

---