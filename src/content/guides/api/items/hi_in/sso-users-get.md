[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users'; creditsCost = 10; api-resource-header-end]

यह रूट SSO यूज़र्स को `100` के पेजों में लौटाता है। पेजिनेशन `skip` पैरामीटर द्वारा प्रदान की जाती है। यूज़र्स को उनके `signUpDate` और `id` के अनुसार सॉर्ट किया जाता है।

[inline-code-attrs-start title = 'SSOUsers cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUsers रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUsers रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    users: SSOUser[]
}
[inline-code-end]