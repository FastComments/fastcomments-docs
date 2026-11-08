[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल `Moderator` जोड़ने की क्षमता प्रदान करता है।

एक `Moderator` बनाने के लिए निम्नलिखित प्रतिबंध हैं:

- एक `name` और `email` हमेशा प्रदान किया जाना चाहिए। `userId` वैकल्पिक है।
- एक `Moderator` बनाते समय निम्नलिखित मान प्रदान नहीं किए जा सकते:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- जब एक `userId` निर्दिष्ट किया जाता है, तो वह उपयोगकर्ता मौजूद होना चाहिए।
- जब एक `userId` निर्दिष्ट किया जाता है, तो वह क्वेरी पैरामीटर में निर्दिष्ट उसी `tenantId` से संबंधित होना चाहिए।
- एक ही टेनेंट में दो मॉडरेटर एक ही `email` के साथ नहीं जोड़े जा सकते।

हम एक ऐसे उपयोगकर्ता के लिए `Moderator` बना सकते हैं जिसके बारे में हमें केवल ईमेल पता हो:

[inline-code-attrs-start title = 'ईमेल द्वारा Moderator बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

या हम अपने टेनेंट से संबंधित एक उपयोगकर्ता के लिए `Moderator` बना सकते हैं, ताकि उनके मॉडरेशन आंकड़ों को ट्रैक किया जा सके:

[inline-code-attrs-start title = 'टेनेंट उपयोगकर्ता द्वारा Moderator बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator बनाने के अनुरोध की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator बनाने की प्रतिक्रिया की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    moderator?: Moderator; // सफलता पर हम पूरा बनाया गया मॉडरेटर लौटाते हैं।
}
[inline-code-end]