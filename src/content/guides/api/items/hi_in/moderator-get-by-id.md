[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

यह रूट id द्वारा एक मॉडरेटर को लौटाता है।

[inline-code-attrs-start title = 'ID द्वारा मॉडरेटर cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'मॉडरेटर रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'मॉडरेटर रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorByIdResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    moderator?: Moderator
}
[inline-code-end]