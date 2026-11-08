[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एकल `TenantUser` को अपडेट करने की क्षमता प्रदान करता है।

एक `TenantUser` को अपडेट करने की निम्नलिखित पाबंदियाँ हैं:

- `signUpDate` भविष्य में नहीं हो सकता।
- `locale` [समर्थित लोकेल](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) की सूची में होना चाहिए।
- `username` पूरे FastComments.com पर अद्वितीय होना चाहिए। यदि यह समस्या है, तो हम SSO का उपयोग करने का सुझाव देते हैं।
- `email` पूरे FastComments.com पर अद्वितीय होना चाहिए। यदि यह समस्या है, तो हम SSO का उपयोग करने का सुझाव देते हैं।
- आप किसी उपयोगकर्ता का `tenantId` अपडेट नहीं कर सकते।

हम एक `TenantUser` इस प्रकार अपडेट कर सकते हैं:

[inline-code-attrs-start title = 'TenantUser अपडेट cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser अपडेट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** जब email या username बदला जाता है, तो आप उपयोगकर्ता की टिप्पणियों को भी अपडेट करने के लिए इसे true पर सेट कर सकते हैं। इससे क्रेडिट लागत दोगुनी हो जाएगी। **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser अपडेट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]