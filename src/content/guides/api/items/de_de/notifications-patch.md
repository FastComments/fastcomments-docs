[api-resource-header-start name = 'Notification'; route = 'PATCH /api/v1/notifications/:id'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, eine `Notification` nach `id` zu aktualisieren.

Das Aktualisieren einer `Notification` hat folgende Einschränkungen:

- Sie können nur die folgenden Felder aktualisieren:
  - `viewed`
  - `optedOut`

[inline-code-attrs-start title = 'Notification PATCH cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/notifications/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"viewed": false,
}'
[inline-code-end]

[inline-code-attrs-start title = 'Notification PATCH Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Notification PATCH Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface NotificationPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
