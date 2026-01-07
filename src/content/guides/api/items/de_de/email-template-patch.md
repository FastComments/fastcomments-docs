[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, eine E-Mail-Vorlage zu aktualisieren, indem nur die ID und die zu aktualisierenden Attribute angegeben werden.

Beachten Sie, dass alle gleichen Validierungen für das Erstellen einer Vorlage auch gelten, zum Beispiel:

- Die Vorlage muss renderbar sein. Dies wird bei jeder Aktualisierung überprüft.
- Sie können keine doppelten Vorlagen für dieselbe Domain haben (ansonsten würde eine stillschweigend ignoriert werden).

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The updated email template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
