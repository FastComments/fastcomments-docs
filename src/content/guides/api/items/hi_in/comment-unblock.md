[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट किसी दिए गए कमेंट को लिखने वाले यूज़र को अन-ब्लॉक करने की सुविधा प्रदान करता है। यह FastComments.com Users, SSO Users, और Tenant Users द्वारा लिखे गए कमेंट्स से अन-ब्लॉकिंग को सपोर्ट करता है।

यह एक `commentIdsToCheck` बॉडी पैरामीटर को सपोर्ट करता है, ताकि यह जांचा जा सके कि इस क्रिया के बाद क्लाइंट पर कोई अन्य संभावित रूप से दिखाई देने वाले कमेंट्स को ब्लॉक/अन-ब्लॉक करना चाहिए या नहीं।

नोट्स:

- यह कॉल हमेशा किसी यूज़र के संदर्भ में की जानी चाहिए। यह यूज़र FastComments.com User, SSO User, या Tenant User हो सकता है।
- रिक्वेस्ट में `userId` वह यूज़र है जो *अन-ब्लॉकिंग कर रहा है*। उदाहरण के लिए: `User A`, `User B` को अन-ब्लॉक करना चाहता है। `userId=User A` पास करें और वह कमेंट आईडी दें जो `User B` ने लिखा था।
- पूरी तरह अनाम कमेंट्स (कोई यूज़र आईडी नहीं, कोई ईमेल नहीं) को ब्लॉक नहीं किया जा सकता और एक एरर रिटर्न किया जाएगा।

[inline-code-attrs-start title = 'Comment Un-Block cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonymous Comment Un-Block cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block रिक्वेस्ट स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block रिस्पॉन्स स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are still blocked. If false, you might want to un-hide the comments from the user so they don't have to refresh. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]