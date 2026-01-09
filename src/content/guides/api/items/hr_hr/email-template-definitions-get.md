[api-resource-header-start name = 'EmailTemplateDefinition'; route = 'GET /api/v1/email-templates/definitions'; creditsCost = 1; api-resource-header-end]

Ovaj API omogućuje dohvaćanje svih objekata `EmailTemplateDefinition`.

[inline-code-attrs-start title = 'Primjer cURL GET za EmailTemplateDefinition'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/definitions?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura GET zahtjeva za EmailTemplateDefinition'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetEmailTemplateDefinitionsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura GET odgovora za EmailTemplateDefinition'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateDefinition {
    name: string
    description: string
    /** Nisu svi predlošci vezani uz domenu. Na primjer, neki administratorski e-mailovi se nikada ne šalju u kontekstu jedne ili više domena. **/
    canBeDomainSpecific: boolean
    emailTemplateId: string
    defaultTestData: object
    defaultTranslationsByLocale: Record<string, Record<string, string>>
    defaultEJS: string
}

interface GetEmailTemplateDefinitionsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    definitions?: EmailTemplateDefinition[]
}
[inline-code-end]

---