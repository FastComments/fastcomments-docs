[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de atualizar um `TenantPackage` por `id`.

A atualização de um `TenantPackage` possui as seguintes restrições:

- Se você estiver definindo `hasFlexPricing` como true, então todos os parâmetros `flex*` são obrigatórios nessa mesma requisição.
- O `name` não pode ser maior que `50 characters`.
- Cada item `forWhoText` não pode ser maior que `200 characters`.
- Cada item `featureTaglines` não pode ser maior que `100 characters`.
- O `TenantPackage` deve ser "menor" que o tenant pai. Por exemplo, todos os parâmetros `max*` devem ter valores menores que o tenant pai. 
- Você não pode alterar o `tenantId` associado com um `TenantPackage`.

[inline-code-attrs-start title = 'Exemplo de requisição cURL PATCH para TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição PATCH de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta PATCH de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'; 
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]

---