[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Ta pot omogoča odstranitev enega `DomainConfig` po ID-ju.

- Note: Odstranitev `DomainConfig` bo preklicala dovoljenje tej domeni za uporabo FastComments.
- Note: Ponovno dodajanje domene prek uporabniškega vmesnika bo znova ustvarilo objekt (z izpolnjeno samo lastnostjo `domain`).

[inline-code-attrs-start title = 'Primer cURL zahteve za odstranitev DomainConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za odstranitev DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za odstranitev DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Vključeno ob neuspehu. **/
    reason?: string
}
[inline-code-end]

---