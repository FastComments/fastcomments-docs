---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[GetTenantPackage_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackage(tenantId = "my-tenant-123", id = "pkg-premium-001")
if response.isSome:
  let pkg = response.get()
  echo pkg
else:
  echo "No package found for tenant"
[inline-code-end]

---