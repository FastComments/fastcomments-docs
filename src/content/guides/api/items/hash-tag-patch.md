[api-resource-header-start name = 'HashTag'; route = 'PATCH /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

This route provides the ability to update a single `HashTag`.

[inline-code-attrs-start title = 'HashTag Update cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-`page`"
}'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag Update Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPatchQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag Update Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPatchResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag' |  'already-exists';
    /** Included on failure. **/
    reason?: string;
    hashTag?: HashTag; // We return the complete updated hashtag on success.
}
[inline-code-end]
