[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Opisuje akreditiv koji vrši zahtev: tenant kojem pripada i, za OAuth pristupne tokene, korisnika koji je autorizovao aplikaciju. Integracije ga koriste za testiranje veze i označavanje.

Sa API ključem odgovor identifikuje samo tenant. Sa OAuth tokenom nosi i korisnika koji je autorizovao i opsege koji su dodeljeni.

[inline-code-attrs-start title = 'Me cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me Struktura Odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju greške. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** Kako je zahtev autentifikovan. **/
    authType: 'api-key' | 'oauth'
    /** Opsezi (scopes) koje akreditiv poseduje. API ključevi poseduju oba. **/
    scopes: ('read' | 'write')[]
    /** Prisutno samo za OAuth tokene. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]