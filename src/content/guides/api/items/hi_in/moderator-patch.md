[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट `id` द्वारा एक `Moderator` को अपडेट करने की क्षमता प्रदान करता है।

एक `Moderator` को अपडेट करने की निम्नलिखित सीमाएँ हैं:

- `Moderator` को अपडेट करते समय निम्नलिखित मान प्रदान नहीं किए जा सकते:
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
- आप एक `Moderator` से जुड़े `tenantId` को नहीं बदल सकते।

[inline-code-attrs-start title = 'Moderator PATCH cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator PATCH प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]