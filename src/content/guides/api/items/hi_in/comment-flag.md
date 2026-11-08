[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट किसी विशिष्ट उपयोगकर्ता के लिए एक टिप्पणी को फ्लैग करने की क्षमता प्रदान करता है।

नोट्स:

- यह कॉल हमेशा एक उपयोगकर्ता के संदर्भ में किया जाना चाहिए। उपयोगकर्ता FastComments.com User, SSO User, या Tenant User हो सकता है।
- यदि एक flag-to-hide थ्रेशोल्ड सेट किया गया है, तो टिप्पणी को परिभाषित संख्या में फ्लैग किए जाने के बाद लाइव स्वचालित रूप से छिपा दिया जाएगा।
- स्वचालित रूप से अन-अप्रूव (छिपाए) जाने के बाद - टिप्पणी को केवल व्यवस्थापक या मॉडरेटर द्वारा ही फिर से अप्रूव किया जा सकता है। अन-फ्लैग करने से टिप्पणी फिर से अप्रूव नहीं होगी।

[inline-code-attrs-start title = 'कमेंट फ्लैग cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

गुमनाम फ्लैगिंग के लिए, हमें एक `anonUserId` निर्दिष्ट करना होगा। यह गुमनाम सत्र का प्रतिनिधित्व करने वाली एक ID हो सकती है, या एक यादृच्छिक UUID।
यह हमें तब भी टिप्पणियों को फ्लैग और अन-फ्लैग करने की सुविधा देता है जब उपयोगकर्ता लॉग इन न हो। इस तरह, जब उसी `anonUserId` के साथ टिप्पणियाँ फ़ेच की जाती हैं, तो टिप्पणी को फ्लैग किया गया चिह्नित किया जा सकता है।

[inline-code-attrs-start title = 'गुमनाम कमेंट फ्लैग cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'कमेंट फ्लैग रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'कमेंट फ्लैग रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
    /** Was the comment un-approved (hidden) due to being flagged too many times? **/
    wasUnapproved?: boolean;
}
[inline-code-end]