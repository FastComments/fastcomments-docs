[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia usunięcie pojedynczego `DomainConfig` według identyfikatora.

- Uwaga: Usunięcie `DomainConfig` spowoduje odebranie autoryzacji tej domenie do korzystania z FastComments.
- Uwaga: Ponowne dodanie domeny przez UI spowoduje ponowne utworzenie obiektu (z wypełnioną jedynie właściwością `domain`).

[inline-code-attrs-start title = 'Przykład żądania cURL usunięcia DomainConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania usunięcia DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usunięcia DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]

---