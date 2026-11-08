[api-resource-header-start name = 'SSOUser'; route = 'PATCH /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एकल SSO यूज़र को अपडेट करने की क्षमता प्रदान करता है।

[inline-code-attrs-start title = 'SSOUser अपडेट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "notfordperfect"
}'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser अपडेट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** जब email या username बदला जाता है, तो आप यूज़र की कमेंट्स को भी अपडेट करने के लिए इसे true सेट कर सकते हैं। इससे क्रेडिट लागत दोगुनी हो जाएगी। **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser अपडेट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-does-not-exist'
    /** विफलता पर शामिल। **/
    reason?: string
    user?: SSOUser; // सफलता पर हम पूरा अपडेट किया गया यूज़र लौटाते हैं।
}
[inline-code-end]