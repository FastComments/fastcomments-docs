[api-resource-header-start name = 'DomainConfig'; route = 'PATCH /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

This API endpoint provides the ability to update a domain configuration by only specifying the domain and the attribute to update.

[inline-code-attrs-start title = 'DomainConfig PATCH cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"emailFromName": "Some New From Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PATCH Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPatchQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PATCH Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPatchResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'domain-does-not-exist' | 'update-would-create-duplicate' | 'domain-too-long' | 'domain-invalid';  
    /** Included on failure. **/
    reason?: string;
    /** The updated configuration. **/
    configuration?: DomainConfig;
}
[inline-code-end]
