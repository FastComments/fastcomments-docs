[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

To API używa paginacji, dostarczanej przez parametr zapytania `page`. EmailTemplates są zwracane w stronach po `100`, posortowane według `createdAt`, a następnie `id`.

[inline-code-attrs-start title = 'Przykład żądania cURL EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Strona do pobrania, zaczynając od 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]

---