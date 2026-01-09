[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

Ta API omogoča pridobitev vseh `DomainConfig` objektov za najemnika.

[inline-code-attrs-start title = 'Primer cURL zahteve za DomainConfig GET'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura GET zahteve DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetDomainConfigsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odziva GET za DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetDomainConfigsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Konfiguracije! **/
    configurations: DomainConfig[] | null
}
[inline-code-end]

---