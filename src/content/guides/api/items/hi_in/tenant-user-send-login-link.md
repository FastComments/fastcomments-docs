[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

यह रूट एक `TenantUser` को लॉगिन लिंक भेजने की सुविधा प्रदान करता है।

यह तब उपयोगी होता है जब आप बैच में यूज़र बना रहे हों और उन्हें FastComments.com पर लॉगिन करने का तरीका न बताना पड़े। यह उन्हें लॉगिन के लिए एक "magic link" भेज देगा जो `30 days` के बाद समाप्त हो जाता है।

किसी `TenantUser` को लॉगिन लिंक भेजने के लिए निम्नलिखित प्रतिबंध मौजूद हैं:
- `TenantUser` पहले से मौजूद होना चाहिए।
- आपके पास उस `Tenant` को प्रबंधित करने की पहुँच होनी चाहिए जिससे `TenantUser` संबंधित है।

हम किसी `TenantUser` को लॉगिन लिंक इस प्रकार भेज सकते हैं:

[inline-code-attrs-start title = 'TenantUser लॉगिन लिंक cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

इससे `Bob at TenantName is inviting you to be a moderator...` जैसा एक ईमेल भेजा जाएगा।

[inline-code-attrs-start title = 'TenantUser लॉगिन लिंक रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser लॉगिन लिंक रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]