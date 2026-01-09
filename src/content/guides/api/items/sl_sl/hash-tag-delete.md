[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Ta ruta omogoča odstranitev uporabnika `HashTag` z navedenim tagom.

Upoštevajte, da če samodejno ustvarjanje `HashTag` ni onemogočeno, lahko uporabniki ponovno ustvarijo hashtage tako, da pri komentiranju vnesejo hashtag.

[inline-code-attrs-start title = 'Primer cURL zahteve za odstranitev HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za odstranitev HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za odstranitev HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** Vključeno ob neuspehu. **/
    reason?: string
}
[inline-code-end]