[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट किसी विशिष्ट उपयोगकर्ता के लिए एक कमेंट को अन-फ्लैग (un-flag) करने की सुविधा प्रदान करता है।

नोट्स:

- यह कॉल हमेशा एक उपयोगकर्ता के संदर्भ में किया जाना चाहिए। उपयोगकर्ता एक FastComments.com User, SSO User, या Tenant User हो सकता है।
- जब कोई कमेंट स्वचालित रूप से अन-अप्रूव (छिपा हुआ) हो जाता है - तो उस कमेंट को केवल एडमिनिस्ट्रेटर या मॉडरेटर द्वारा ही दोबारा अप्रूव किया जा सकता है। अन-फ्लैग करने से कमेंट दोबारा अप्रूव नहीं होगा।

[inline-code-attrs-start title = 'कमेंट अन-फ्लैग cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

गुमनाम (anonymous) फ्लैगिंग के लिए, हमें एक `anonUserId` निर्दिष्ट करना होगा। यह गुमनाम सत्र (anonymous session) का प्रतिनिधित्व करने वाली एक ID हो सकती है, या एक रैंडम UUID।

[inline-code-attrs-start title = 'गुमनाम कमेंट फ्लैग cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'कमेंट अन-फ्लैग रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'कमेंट अन-फ्लैग रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]