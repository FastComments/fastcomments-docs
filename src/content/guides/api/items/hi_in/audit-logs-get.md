[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

यह API पेजिनेशन का उपयोग करती है, जो `skip`, `limit`, `before`, और `after` पैरामीटर द्वारा प्रदान की जाती है। AuditLogs डिफ़ॉल्ट रूप से `1000` के पेजों में लौटाए जाते हैं, अधिकतम `limit` `10000` तक, `when` और `id` के अनुसार क्रमबद्ध। पेज बड़े होते हैं क्योंकि इस एंडपॉइंट का उपयोग आमतौर पर इतिहास को इंटरैक्टिव रूप से पेज करने के बजाय डंप करने के लिए किया जाता है।

हर `100` लॉग लौटाने पर क्रेडिट लागत `1` होती है।

डिफ़ॉल्ट रूप से, आपको **सबसे नए आइटम पहले** के साथ एक सूची प्राप्त होगी। इस तरह, आप `skip=0` से शुरू करके पोल कर सकते हैं और आखिरी उपभोग किए गए रिकॉर्ड को खोजने तक पेजिनेट कर सकते हैं।

वैकल्पिक रूप से, आप सबसे पुराने पहले के क्रम में सॉर्ट कर सकते हैं, और जब तक कोई और रिकॉर्ड न बचे तब तक पेजिनेट कर सकते हैं।

सॉर्टिंग `order` को `ASC` या `DESC` में सेट करके की जा सकती है। डिफ़ॉल्ट `DESC` है।

तारीख के अनुसार क्वेरी `before` और `after` के माध्यम से मिलीसेकंड के साथ टाइमस्टैम्प के रूप में संभव है। `before` और `after` समावेशी (inclusive) नहीं हैं, और इनमें से कोई भी अकेले उपयोग किया जा सकता है।

## किसी व्यक्ति के साथ क्या हुआ, यह खोजना

हर इवेंट रिकॉर्ड करता है कि उसे किसने किया (`username`, `userId`, `ip`) और, अलग से, वह किस पर किया गया। `targetLabel` उस ऑब्जेक्ट का एक मानव-पठनीय लेबल है, उदाहरण के लिए `jsmith (jsmith@example.com)`, और `targetId` उसकी id है। जब आप किसी व्यक्ति का नाम या ईमेल जानते हों लेकिन उनकी id नहीं, तो लेबल पर केस-असंवेदनशील सबस्ट्रिंग मैच के लिए `target` का उपयोग करें।

डिलीट इवेंट के समय का लेबल कैप्चर करते हैं, इसलिए मूल रिकॉर्ड हट जाने के बाद भी हटाए गए उपयोगकर्ता या मॉडरेटर की पहचान की जा सकती है।

## प्रबंधित टेनेंट

यदि आपका टेनेंट अन्य टेनेंट प्रबंधित करता है, तो अपने टेनेंट और उसके द्वारा प्रबंधित हर टेनेंट के इवेंट एक ही प्रतिक्रिया में प्राप्त करने के लिए `includeManagedTenants=true` सेट करें। प्रत्येक लौटाए गए लॉग की `tenantId` बताती है कि वह किस टेनेंट से आया।

[inline-code-attrs-start title = 'AuditLog cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** अधिकतम 10000। डिफ़ॉल्ट 1000। **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** केवल इस username द्वारा किए गए इवेंट। **/
    username?: string
    /** केवल इस IP पते से इवेंट। **/
    ip?: string
    /** केवल इस प्रकार के इवेंट। **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** केवल इस रिसोर्स के इवेंट, जैसे User या Moderator। **/
    resourceName?: string
    /** केवल उन इवेंट के लिए जिनकी प्रभावित ऑब्जेक्ट की यह id है। **/
    targetId?: string
    /** प्रभावित ऑब्जेक्ट के लेबल पर केस-असंवेदनशील सबस्ट्रिंग मैच। **/
    target?: string
    /** इस टेनेंट द्वारा प्रबंधित टेनेंट के इवेंट भी लौटाएं। **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** विफलता पर शामिल। **/
    reason?: string
    /** लॉग! **/
    auditLogs: AuditLog[]
}
[inline-code-end]