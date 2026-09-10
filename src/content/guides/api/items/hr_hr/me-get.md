[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Opisuje vjerodajnicu koja šalje zahtjev: najmoda kojem pripada i, za OAuth pristupne tokene, 
korisnika koji je autorizirao aplikaciju. Integracije ga koriste za testiranje veze i označavanje.

Kod API ključa odgovor identificira samo najmodu. Kod OAuth tokena nosi i 
korisnika koji je autorizirao i opsege koji su odobreni.

[inline-code-attrs-start title = 'Primjer cURL za Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno pri neuspjehu. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** Kako je zahtjev autentificiran. **/
    authType: 'api-key' | 'oauth'
    /** Opsezi koje vjerodajnica posjeduje. API ključevi posjeduju oba. **/
    scopes: ('read' | 'write')[]
    /** Prisutno samo za OAuth tokene. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]

---