[api-resource-header-start name = 'QuestionConfig'; route = 'DELETE /api/v1/question-configs/:id'; creditsCost = 100; api-resource-header-end]

Ovaj endpoint omogućava brisanje `QuestionConfig` po id-u.

**Ovo će obrisati sve odgovarajuće rezultate pitanja (ali ne i komentare).** Ovo je deo visoke cene u kreditima.

[inline-code-attrs-start title = 'Primer cURL zahteva za brisanje QuestionConfig-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/question-configs/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za brisanje QuestionConfig-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje QuestionConfig-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]

---