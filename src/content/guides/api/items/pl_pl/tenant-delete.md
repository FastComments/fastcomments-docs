[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Ten endpoint umożliwia usunięcie `Tenant` **i wszystkich powiązanych danych** (użytkowników, komentarzy itp.) na podstawie id.

Istnieją następujące ograniczenia dotyczące usuwania tenantów:

- Tenant musi być Twój własny lub white-labeled tenant, którym zarządzasz.
- Parametr zapytania `sure` musi być ustawiony na `true`.

[inline-code-attrs-start title = 'Przykład cURL usunięcia Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania usunięcia Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usunięcia Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]