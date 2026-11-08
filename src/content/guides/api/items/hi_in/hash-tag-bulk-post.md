[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags/bulk'; creditsCost = 1; api-resource-header-end]

यह रूट एक साथ 100 `HashTag` ऑब्जेक्ट तक जोड़ने की क्षमता प्रदान करता है।

[inline-code-attrs-start title = 'HashTag बल्क क्रिएट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'HashTag बल्क क्रिएट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag बल्क क्रिएट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagBulkPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** विफलता पर शामिल। **/
    reason?: string
    results?: HashTagPostResponse[]; // हम प्रत्येक प्रदान किए गए टैग के लिए HashTagPostResponse ऑब्जेक्ट्स की सूची लौटाते हैं।
}
[inline-code-end]