[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Pojedinačne EmailTemplates mogu se dohvatiti prema njihovom odgovarajućem `id`-u (NE `emailTemplateId`).

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za EmailTemplate prema ID-u'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za EmailTemplate prema ID-u'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesByIdRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za EmailTemplate prema ID-u'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    emailTemplate?: EmailTemplate | null
}
[inline-code-end]

---