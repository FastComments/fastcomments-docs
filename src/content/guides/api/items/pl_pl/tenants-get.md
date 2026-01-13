[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

To API zwraca tenantów, którymi zarządza Twój tenant.

Paginacja jest realizowana za pomocą parametru zapytania `skip`. Tenanci są zwracani stronami po `100`, posortowane według `signUpDate` i `id`.

Koszt jest naliczany na podstawie liczby zwróconych tenantów, wynosząc `1 credit per 10` zwróconych tenantów.

[inline-code-attrs-start title = 'Przykład cURL dla Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Możesz zdefiniować parametry `meta` na obiektach `Tenant` i wyszukiwać pasujących tenantów. Na przykład, dla klucza `someKey` i wartości meta `some-value`, możemy skonstruować obiekt JSON z tą parą klucz/wartość, a następnie zakodować go do URI jako parametr zapytania, aby przefiltrować:

[inline-code-attrs-start title = 'Przykład cURL zapytania Tenant według meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Liczba tenantów do pominięcia w paginacji. **/
    skip?: number
    /** Filtrowanie według parametrów meta. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]