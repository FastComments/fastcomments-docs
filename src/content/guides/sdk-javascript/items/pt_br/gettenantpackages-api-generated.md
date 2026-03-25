## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f8e3b4c";
const skip: number = 20;
const packagesDefault: GetTenantPackages200Response = await getTenantPackages(tenantId);
const packagesWithSkip: GetTenantPackages200Response = await getTenantPackages(tenantId, skip);
[inline-code-end]

---