[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Deze route biedt het verwijderen van een enkele `DomainConfig` op basis van id.

- Opmerking: Het verwijderen van een `DomainConfig` zal dat domein niet langer machtigen om FastComments te gebruiken.
- Opmerking: Het opnieuw toevoegen van een domein via de UI zal het object opnieuw aanmaken (met alleen `domain` ingevuld).

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor DomainConfig-verwijdering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van DomainConfig-verwijderingsverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van DomainConfig-verwijderingsantwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]