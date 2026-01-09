[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ovaj API koristi paginaciju, koju obezbeđuju parametri `skip`, `before` i `after`. AuditLogs se vraćaju u stranicama od po `1000`, sortirani po `when` i `id`.

Preuzimanje svakih `1000` zapisa košta `10` kredita.

Po podrazumevanoj vrednosti dobićete listu sa **najnovijim stavkama na vrhu**. Na ovaj način možete započeti polling sa `skip=0`, paginirati dok ne nađete poslednji zapis koji ste obradili.

Alternativno, možete sortirati od najstarijih i paginirati dok ne bude više zapisa.

Sortiranje se radi podešavanjem `order` na `ASC` ili `DESC`. Podrazumevano je `ASC`.

Pretraga po datumu je moguća pomoću `before` i `after` kao timestamp-a u milisekundama. `before` i `after` nisu uključivi.

[inline-code-attrs-start title = 'Primer cURL zahteva za AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Logovi! **/
    auditLogs: AuditLog[]
}
[inline-code-end]