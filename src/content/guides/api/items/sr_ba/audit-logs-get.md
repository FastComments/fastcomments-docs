[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ovaj API koristi paginaciju, koja se obezbjeđuje parametrima `skip`, `before`, i `after`. AuditLog zapisi se vraćaju u stranicama po `1000`, poredani po `when` i `id`.

Dohvatanje svake `1000` zapisa ima trošak kredita od `10`.

Po zadanim postavkama dobićete listu s **najnovijim stavkama na početku**. Na ovaj način možete započeti preuzimanje sa `skip=0`, paginirati dok ne nađete posljednji zapis koji ste obradili.

Alternativno, možete sortirati od najstarijih prema najnovijim, i paginirati dok ne bude više zapisa.

Sortiranje se može uraditi postavljanjem `order` na `ASC` ili `DESC`. Zadano je `ASC`.

Upit po datumu je moguć putem `before` i `after` kao timestampova sa milisekundama. `before` i `after` NISU uključivi.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Logovi! **/
    auditLogs: AuditLog[]
}
[inline-code-end]