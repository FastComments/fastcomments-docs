[api-resource-header-start name = 'DomainConfig'; route = 'POST /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, Domain-Konfigurationen zu erstellen.

Das Hinzufügen einer Konfiguration für eine Domain autorisiert diese Domain für das FastComments-Konto.

Häufige Anwendungsfälle dieser API sind die Ersteinrichtung, wenn viele Domains hinzugefügt werden sollen, oder benutzerdefinierte Konfigurationen für den E-Mail-Versand.

[inline-code-attrs-start title = 'DomainConfig POST cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"domain": "example.com",
	"emailFromName": "some from name",
	"emailFromEmail": "some@test.com",
	"logoSrc": "https://example.com/my-logo-big.png",
	"logoSrc100px": "https://example.com/my-logo-100px.png",
	"footerUnsubscribeURL": "http://example.com/unsubscribe-ui",
	"emailHeaders": {
		"List-Unsubscribe-Post": "List-Unsubscribe=One-Click",
		"List-Unsubscribe": "<https://example.com/opt-out/[userId]>"
	}
}'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig POST Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig POST Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'configuration-exists-for-domain' | 'domain-too-long' | 'domain-invalid';
    /** Included on failure. **/
    reason?: string
    /** The created configuration. **/
    configuration?: DomainConfig
}
[inline-code-end]
