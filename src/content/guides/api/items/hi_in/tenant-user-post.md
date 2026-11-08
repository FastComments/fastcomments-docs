[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल `TenantUser` जोड़ने की क्षमता प्रदान करता है।

एक `TenantUser` बनाने के लिए निम्नलिखित प्रतिबंध हैं:

- एक `username` आवश्यक है।
- एक `email` आवश्यक है।
- `signUpDate` भविष्य की तारीख नहीं हो सकती।
- `locale` [समर्थित लोकेल्स](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) की सूची में होना चाहिए।
- `username` पूरे FastComments.com पर अद्वितीय होना चाहिए। यदि यह समस्या है, तो हम SSO का उपयोग करने का सुझाव देते हैं।
- `email` पूरे FastComments.com पर अद्वितीय होना चाहिए। यदि यह समस्या है, तो हम SSO का उपयोग करने का सुझाव देते हैं।
- आप अपने पैकेज में `maxTenantUsers` के अंतर्गत परिभाषित सीमा से अधिक टेनेंट यूज़र नहीं बना सकते।

हम एक `TenantUser` इस प्रकार बना सकते हैं:

[inline-code-attrs-start title = 'TenantUser बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser बनाने की रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser बनाने की रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]