[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

फ़ीड में पोस्ट प्राप्त करता है, नवीनतम पहले। पेजिनेशन कर्सर आधारित है: अगला पेज प्राप्त करने के लिए आपके द्वारा प्राप्त अंतिम पोस्ट की `_id` को `afterId` के रूप में पास करें।

प्रत्येक दस लौटाई गई पोस्ट पर एक क्रेडिट लागत होती है, न्यूनतम एक क्रेडिट।

[inline-code-attrs-start title = 'FeedPost cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** इस पोस्ट आईडी के बाद की पोस्ट लौटाएँ। पहले पेज के लिए छोड़ें। **/
    afterId?: string
    /** डिफ़ॉल्ट 10। अधिकतम 1000। **/
    limit?: number
    /** केवल इन टैग्स वाली पोस्ट लौटाएँ। एक से अधिक टैग के लिए पैरामीटर दोहराएँ। **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** विफलता पर शामिल। **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]