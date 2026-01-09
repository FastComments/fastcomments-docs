[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ta API uporablja straničenje, ki ga zagotavlja poizvedni parameter `page`. EmailTemplates so vrnjeni po straneh po `100`, urejeni po `createdAt` in nato `id`.

[inline-code-attrs-start title = 'Primer cURL za EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Stran za pridobitev, začne se z 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]