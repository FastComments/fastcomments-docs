[api-resource-header-start name = 'HashTag'; route = 'PATCH /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Esta rota permite atualizar um único `HashTag`.

[inline-code-attrs-start title = 'Exemplo cURL de atualização de HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-`page`"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de requisição de atualização de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de resposta de atualização de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag' |  'already-exists'
    /** Incluído em caso de falha. **/
    reason?: string
    hashTag?: HashTag; // Retornamos a hashtag completa atualizada em caso de sucesso.
}
[inline-code-end]

---