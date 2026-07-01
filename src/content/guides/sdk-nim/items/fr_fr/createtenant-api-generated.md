## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| createTenantBody | CreateTenantBody | Non |  |

## Réponse

Retourne : [`Option[CreateTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_response.nim)

## Exemple

[inline-code-attrs-start title = 'createTenant Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.createTenant(tenantId = "my-tenant-123", createTenantBody = CreateTenantBody())
if maybeResp.isSome:
  let resp = maybeResp.get()
  discard resp
discard httpResp
[inline-code-end]