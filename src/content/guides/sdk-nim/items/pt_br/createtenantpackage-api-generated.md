## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| createTenantPackageBody | CreateTenantPackageBody | Não |  |

## Resposta

Retorna: [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## Exemplo

[inline-code-attrs-start title = 'createTenantPackage Exemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.createTenantPackage(
  tenantId = "my-tenant-123",
  createTenantPackageBody = CreateTenantPackageBody()
)

if responseOpt.isSome:
  let response = responseOpt.get()
[inline-code-end]