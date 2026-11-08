[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

वोट को `urlId` द्वारा फ़ेच किया जाना चाहिए।

### वोट के प्रकार

तीन प्रकार के वोट होते हैं:

- प्रमाणित वोट (Authenticated Votes), जो संबंधित कमेंट पर लागू होते हैं। आप इन्हें इस API के माध्यम से बना सकते हैं।
- प्रमाणित वोट, जो सत्यापन के लिए **लंबित (pending)** हैं, और इस प्रकार अभी तक कमेंट पर लागू नहीं हुए हैं। ये तब बनाए जाते हैं जब कोई उपयोगकर्ता FastComments.com के *login to vote* तंत्र का उपयोग करता है।
- अनाम वोट (Anonymous Votes), जो संबंधित कमेंट पर लागू होते हैं। ये अनाम कमेंटिंग के साथ बनाए जाते हैं।

भ्रम को कम करने के लिए इन्हें API में अलग-अलग सूचियों में लौटाया जाता है।

[inline-code-attrs-start title = 'वोट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'वोट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'वोट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** विफलता पर शामिल। **/
    reason?: string
    /** प्राधिकृत, सत्यापित वोट, जो उनके संबंधित कमेंट्स पर लागू हैं। **/
    appliedAuthorizedVotes: Vote[]
    /** अनाम वोट, जो उनके संबंधित कमेंट्स पर लागू हैं। **/
    appliedAnonymousVotes: Vote[]
    /** सत्यापन के लिए लंबित वोट, जो अभी तक उनके संबंधित कमेंट्स पर लागू नहीं हुए हैं। **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### अनाम वोट संबंधी नोट्स

ध्यान दें कि इस API के माध्यम से बनाए गए अनाम वोट `appliedAuthorizedVotes` सूची में दिखाई देंगे। उन्हें प्राधिकृत माना जाता है क्योंकि उन्हें API के माध्यम से API कुंजी के साथ बनाया गया था।

`appliedAnonymousVotes` संरचना उन वोटों के लिए है जो बिना ईमेल, API कुंजी आदि के बनाए गए थे।