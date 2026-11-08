[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

यह रूट दिए गए टैग द्वारा एक `HashTag` उपयोगकर्ता को हटाने की सुविधा प्रदान करता है।

ध्यान दें कि जब तक स्वचालित `HashTag` निर्माण अक्षम नहीं किया गया है, टिप्पणी करते समय हैशटैग प्रदान करके उपयोगकर्ता द्वारा हैशटैग को फिर से बनाया जा सकता है।

[inline-code-attrs-start title = 'HashTag हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag हटाने के अनुरोध की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag हटाने की प्रतिक्रिया की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]