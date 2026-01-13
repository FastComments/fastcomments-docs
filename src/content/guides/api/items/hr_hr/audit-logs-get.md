[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ovaj API koristi paginaciju, koju omogućuju parametri `skip`, `before` i `after`. AuditLogs se vraćaju na stranicama od `1000`, poredani po `when` i `id`.

Dohvaćanje svakih `1000` zapisnika ima trošak kredita od `10`.

Prema zadanim postavkama, primit ćete popis s **najnovijim stavkama na prvom mjestu**. Na taj način možete početi s `skip=0`, paginirajući dok ne pronađete zadnji zapis koji ste konzumirali.

Alternativno, možete sortirati od najstarijeg prema naprijed i paginirati dok nema više zapisa.

Sortiranje se može izvršiti postavljanjem `order` na `ASC` ili `DESC`. Zadano je `ASC`.

Upit prema datumu moguć je putem `before` i `after` kao vremenskih oznaka s milisekundama. `before` i `after` NISU uključivi.

[inline-code-attrs-start title = 'Primjer cURL za AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]
