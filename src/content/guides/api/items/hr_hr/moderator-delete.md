[api-resource-header-start name = 'Moderator'; route = 'DELETE /api/v1/moderators/:id'; creditsCost = 5; api-resource-header-end]

Ovaj endpoint omogućuje uklanjanje `Moderator`a prema njegovom id-u.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje Moderator-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje Moderator-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje Moderator-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno kod neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found'
    /** Uključeno kod neuspjeha. **/
    reason?: string
}
[inline-code-end]

---