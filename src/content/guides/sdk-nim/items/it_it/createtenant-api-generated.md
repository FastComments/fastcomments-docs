## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | No |  |

## Risposta

Restituisce: [`Option[CreateTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio createTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.createTenant(tenantId = "my-tenant-123", createTenantBody = CreateTenantBody())
if maybeResp.isSome:
  let resp = maybeResp.get()
  discard resp
discard httpResp
[inline-code-end]