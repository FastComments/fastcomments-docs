[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

यह रूट id द्वारा एकल Tenant लौटाता है।

[inline-code-attrs-start title = 'ID द्वारा Tenant cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantByIdResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'unauthorized'
    /** विफलता पर शामिल। **/
    reason?: string
    tenant?: Tenant
}
[inline-code-end]