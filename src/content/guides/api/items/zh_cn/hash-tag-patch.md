[api-resource-header-start name = 'HashTag'; route = 'PATCH /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

此路由允许更新单个 `HashTag`。

[inline-code-attrs-start title = 'HashTag 更新 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-`page`"
}'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag 更新 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag 更新 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPatchResponse {
    status: 'success' | 'failed'
    /** 仅在失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag' |  'already-exists'
    /** 仅在失败时包含。 **/
    reason?: string
    hashTag?: HashTag; // 成功时返回完整更新后的 hashtag.
}
[inline-code-end]