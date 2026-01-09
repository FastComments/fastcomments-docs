[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje dodavanje jednog `HashTag`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za stvaranje HashTaga'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "tag": "#example",
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za stvaranje HashTaga'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za stvaranje HashTaga'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    hashTag?: HashTag; // Vraćamo kompletan kreirani hashtag u slučaju uspjeha.
}
[inline-code-end]