## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | No |  |

## Response

מחזיר: [`Option[CreateTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.createTenant(tenantId = "my-tenant-123", createTenantBody = CreateTenantBody())
if maybeResp.isSome:
  let resp = maybeResp.get()
  discard resp
discard httpResp
[inline-code-end]