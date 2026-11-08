[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एकल `TenantUser` को बदलने की क्षमता प्रदान करता है।

एक `TenantUser` को बदलने की निम्नलिखित सीमाएँ हैं:

- `signUpDate` भविष्य में नहीं हो सकता।
- `locale` [समर्थित लोकेल](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) की सूची में होना चाहिए।
- `username` पूरे FastComments.com में अद्वितीय होना चाहिए। यदि यह समस्या है, तो हम SSO का उपयोग करने का सुझाव देते हैं।
- `email` पूरे FastComments.com में अद्वितीय होना चाहिए। यदि यह समस्या है, तो हम SSO का उपयोग करने का सुझाव देते हैं।
- आप किसी उपयोगकर्ता का `tenantId` अपडेट नहीं कर सकते।

हम एक `TenantUser` इस प्रकार बना सकते हैं

[inline-code-attrs-start title = 'TenantUser बदलने का cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser बदलने का अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** जब email या username बदला जाता है, तो आप इसे true सेट करके उपयोगकर्ता की टिप्पणियों को भी अपडेट कर सकते हैं। इससे क्रेडिट लागत दोगुनी हो जाएगी। **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser बदलने की प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]