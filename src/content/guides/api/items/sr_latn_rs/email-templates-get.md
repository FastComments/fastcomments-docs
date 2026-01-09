[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ovaj API koristi paginaciju, obezbeđenu parametrom upita `page`. EmailTemplates se vraćaju u stranicama po `100`, sortirane po `createdAt` i zatim po `id`.

[inline-code-attrs-start title = 'EmailTemplate cURL primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Stranica za preuzimanje, počinje od 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju greške. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]