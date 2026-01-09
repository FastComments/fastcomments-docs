[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava uklanjanje pojedinačnog `DomainConfig` po id-u.

- Napomena: Uklanjanjem `DomainConfig` autorizacija te domene za korištenje FastComments će biti opozvana.
- Napomena: Ponovnim dodavanjem domene preko UI-ja objekt će biti ponovo kreiran (sa samo poljem `domain` popunjenim).

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje DomainConfig-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje DomainConfig-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje DomainConfig-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]