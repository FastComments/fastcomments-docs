[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava uklanjanje `HashTag`-a na osnovu proslijeđenog taga.

Imajte na umu da, osim ako automatsko kreiranje `HashTag`-ova nije onemogućeno, hashtagovi mogu biti ponovo kreirani od strane korisnika koji uključe hashtag prilikom komentarisanja.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje HashTag-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje HashTag-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora pri uklanjanju HashTag-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]