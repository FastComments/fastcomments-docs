[api-resource-header-start name = 'EmailTemplate'; route = 'DELETE /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

यह रूट id द्वारा एकल `EmailTemplate` को हटाने की सुविधा प्रदान करता है।

[inline-code-attrs-start title = 'EmailTemplate हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate हटाने के अनुरोध की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate हटाने की प्रतिक्रिया की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]