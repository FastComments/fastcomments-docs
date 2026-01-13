[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Deze route verwijdert een `HashTag` op basis van de opgegeven tag.

Houd er rekening mee dat, tenzij automatische creatie van `HashTag` is uitgeschakeld, hashtags opnieuw kunnen worden aangemaakt wanneer een gebruiker de hashtag opgeeft bij het plaatsen van een reactie.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor HashTag-verwijdering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het verzoek voor HashTag-verwijdering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het antwoord op HashTag-verwijdering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij falen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** Opgenomen bij falen. **/
    reason?: string
}
[inline-code-end]

---