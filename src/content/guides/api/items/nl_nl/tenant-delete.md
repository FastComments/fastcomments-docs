[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Deze route zorgt voor het verwijderen van een `Tenant` **en alle bijbehorende gegevens** (gebruikers, reacties, etc.) op basis van id.

De volgende beperkingen gelden bij het verwijderen van tenants:

- De tenant moet van uzelf zijn, of een white-labeled tenant die u beheert.
- De queryparameter `sure` moet op `true` worden gezet.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor tenantverwijdering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van verzoek voor tenantverwijdering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het antwoord voor tenantverwijdering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---