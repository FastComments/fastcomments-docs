[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

यह रूट id द्वारा एक `Tenant` **और उससे जुड़े सभी डेटा** (users, comments, आदि) को हटाने की सुविधा प्रदान करता है।

Tenants को हटाने के लिए निम्नलिखित प्रतिबंध लागू हैं:

- Tenant आपका अपना होना चाहिए, या ऐसा white labeled tenant होना चाहिए जिसे आप प्रबंधित करते हैं।
- `sure` query parameter को `true` पर सेट होना चाहिए।

[inline-code-attrs-start title = 'Tenant हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant हटाने के अनुरोध की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant हटाने के प्रतिक्रिया की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]