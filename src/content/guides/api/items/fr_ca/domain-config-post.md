[api-resource-header-start name = 'DomainConfig'; route = 'POST /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

Ce endpoint API fournit la capacité de créer des configurations de domaine.

Ajouter une configuration pour un domaine autorise ce domaine pour le compte FastComments.

Les cas d'utilisation courants de cette API sont la configuration initiale, si de nombreux domaines doivent être ajoutés, ou une configuration personnalisée pour l'envoi d'emails.

[inline-code-attrs-start title = 'Exemple cURL de POST de DomainConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structure de Requête POST de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse POST de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
