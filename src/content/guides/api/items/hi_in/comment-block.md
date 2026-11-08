[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट किसी दिए गए कमेंट को लिखने वाले यूज़र को ब्लॉक करने की सुविधा प्रदान करता है। यह FastComments.com Users, SSO Users, और Tenant Users द्वारा लिखे गए कमेंट्स से ब्लॉकिंग को सपोर्ट करता है।

यह एक `commentIdsToCheck` बॉडी पैरामीटर को सपोर्ट करता है, जिससे यह जांचा जा सके कि इस एक्शन के बाद क्लाइंट पर मौजूद कोई अन्य संभावित रूप से दिखाई देने वाले कमेंट्स को ब्लॉक/अनब्लॉक किया जाना चाहिए या नहीं।

नोट्स:

- यह कॉल हमेशा किसी यूज़र के संदर्भ (context) में की जानी चाहिए। यूज़र FastComments.com User, SSO User, या Tenant User हो सकता है।
- रिक्वेस्ट में `userId` वह यूज़र है जो *ब्लॉक कर रहा है*। उदाहरण के लिए: `User A`, `User B` को ब्लॉक करना चाहता है। तो `userId=User A` और `User B` द्वारा लिखे गए कमेंट की id पास करें।
- पूरी तरह से अनाम (anonymous) कमेंट्स (कोई user id नहीं, कोई email नहीं) को ब्लॉक नहीं किया जा सकता और एक एरर रिटर्न किया जाएगा।

[inline-code-attrs-start title = 'Comment Block cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

अनाम (anonymous) ब्लॉकिंग के लिए, हमें एक `anonUserId` निर्दिष्ट करना होगा। यह अनाम सेशन को दर्शाने वाली एक ID हो सकती है, या एक रैंडम UUID।
इससे हम उन कमेंट्स को ब्लॉक करने की सुविधा दे पाते हैं जिन्हें उसी `anonUserId` से फ़ेच करके तब भी ब्लॉक किया जा सके जब यूज़र लॉग इन न हो।

[inline-code-attrs-start title = 'Anonymous Comment Block cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Block रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Block रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are also blocked. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]