[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल SSO यूज़र को अपडेट करने की सुविधा प्रदान करता है।

[inline-code-attrs-start title = 'SSOUser अपडेट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

इस उदाहरण में हम एक्सेस कंट्रोल के लिए `groupIds` निर्दिष्ट करते हैं, लेकिन यह वैकल्पिक है।

[inline-code-attrs-start title = 'SSOUser अपडेट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** जब email या username बदला जाता है, तो आप इसे true सेट करके यूज़र की कमेंट्स को भी अपडेट कर सकते हैं। इससे क्रेडिट लागत दोगुनी हो जाएगी। **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser अपडेट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** विफलता पर शामिल। **/
    reason?: string
    user?: SSOUser; // सफलता पर हम अपडेट किया गया यूज़र लौटाते हैं।
}
[inline-code-end]