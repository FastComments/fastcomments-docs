[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

किसी दिए गए `urlId` पर एक उपयोगकर्ता द्वारा छोड़े गए वोट प्राप्त करने की सुविधा देता है। यह एक `userId` लेता है जो कोई भी FastComments.com या `SSO User` हो सकता है।

यह तब उपयोगी है जब आप यह दिखाना चाहें कि उपयोगकर्ता ने किसी कमेंट पर वोट किया है या नहीं। कमेंट प्राप्त करते समय, बस उसी समय उसी `urlId` के लिए उपयोगकर्ता के लिए इस API को कॉल करें।

यदि आप गुमनाम (anonymous) वोटिंग का उपयोग कर रहे हैं, तो आप `userId` के स्थान पर `anonUserId` पास करना चाहेंगे।

[inline-code-attrs-start title = 'Votes For User cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Votes For Anon User cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

ध्यान दें कि गुमनाम वोट `appliedAuthorizedVotes` सूची में दिखाई देंगे। उन्हें अधिकृत माना जाता है क्योंकि उन्हें API के माध्यम से API कुंजी के साथ बनाया गया था।

[inline-code-attrs-start title = 'Votes For User रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Votes For User रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]