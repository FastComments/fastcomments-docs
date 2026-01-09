[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags/bulk'; creditsCost = 1; api-resource-header-end]

Ta ścieżka umożliwia dodanie do 100 `HashTag` obiektów jednocześnie.

[inline-code-attrs-start title = 'Przykład cURL — masowe tworzenie HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/hash-tags/bulk?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "tags": [
        {
            "tag": "#example",
            "url": "https://example.com/some-page"
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania — masowe tworzenie HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi — masowe tworzenie HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagBulkPostResponse {
    status: 'success' | 'failed'
    /** Zwracane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** Zwracane w przypadku niepowodzenia. **/
    reason?: string
    results?: HashTagPostResponse[]; // Zwracamy listę obiektów HashTagPostResponse dla każdego dostarczonego tagu.
}
[inline-code-end]