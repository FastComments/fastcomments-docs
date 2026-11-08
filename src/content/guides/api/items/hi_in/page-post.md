[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट पेज बनाने की क्षमता प्रदान करता है।

एक सामान्य उपयोग का मामला एक्सेस कंट्रोल है।

नोट्स:

- यदि आपने किसी कमेंट थ्रेड पर कमेंट किया है, या `Comment` बनाने के लिए API को कॉल किया है, तो आप पहले से ही एक `Page` ऑब्जेक्ट बना चुके हैं! आप इसे `/by-url-id` `Page` रूट के माध्यम से फ़ेच करने का प्रयास कर सकते हैं, जिसमें कमेंट विजेट को दिया गया वही `urlId` पास करें।
- `Page` संरचना में कुछ **गणना किए गए** मान होते हैं।
  वर्तमान में, ये `commentCount` और `rootCommentCount` हैं।
  इन्हें स्वचालित रूप से पॉपुलेट किया जाता है और API द्वारा सेट नहीं किया जा सकता। ऐसा करने का प्रयास करने पर API एक त्रुटि लौटाएगा।

[inline-code-attrs-start title = 'Page POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page POST रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** विफलता पर शामिल। **/
    reason?: string
    /** बनाया गया पेज। **/
    page?: Page
}
[inline-code-end]