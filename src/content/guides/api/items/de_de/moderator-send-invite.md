[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Diese Route bietet die Möglichkeit, einen einzelnen `Moderator` einzuladen.

Die folgenden Einschränkungen gelten für das Senden einer Einladungs-E-Mail an einen `Moderator`:
- Der `Moderator` muss bereits existieren.
- Der `fromName` darf nicht länger als `100 Zeichen` sein.

**Hinweise:**
- Wenn ein Benutzer mit der angegebenen E-Mail bereits existiert, wird er eingeladen, die Kommentare Ihres Tenants zu moderieren.
- Wenn ein Benutzer mit der angegebenen E-Mail **nicht existiert**, führt der Einladungslink ihn durch die Erstellung seines Kontos.
- Die Einladung läuft nach `30 Tagen` ab.

Wir können einen `Moderator` für einen Benutzer erstellen, von dem wir nur die E-Mail kennen:

[inline-code-attrs-start title = 'Moderator Einladen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Dies sendet eine E-Mail wie `Bob bei TenantName lädt Sie ein, Moderator zu werden...`

[inline-code-attrs-start title = 'Moderator Einladen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** The email sent to the user will appear to be sent from this name. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Einladen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
