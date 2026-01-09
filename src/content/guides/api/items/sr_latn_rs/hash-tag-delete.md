[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava uklanjanje `HashTag` korisnika po prosleđenom tagu.

Imajte na umu da, osim ako automatsko kreiranje `HashTag` nije onemogućeno, heštegovi mogu biti ponovo kreirani od strane korisnika koji navede hešteg prilikom komentarisanja.

[inline-code-attrs-start title = 'Primer cURL zahteva za uklanjanje HashTag-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za uklanjanje HashTag-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje HashTag-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]

---