[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल `HashTag` जोड़ने की क्षमता प्रदान करता है।

[inline-code-attrs-start title = 'HashTag बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "tag": "#example",
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag बनाने के अनुरोध की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag बनाने के प्रतिक्रिया की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** विफलता पर शामिल। **/
    reason?: string
    hashTag?: HashTag; // सफलता पर हम पूरा बनाया गया हैशटैग लौटाते हैं।
}
[inline-code-end]