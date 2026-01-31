## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | No |  |

## Response

Returns: [`Option[CreateTenant_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant200response.nim)

## Example

[inline-code-attrs-start title = 'createTenant Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenant(tenantId = "my-tenant-123", createTenantBody = default(CreateTenantBody))
if response.isSome:
  let tenant = response.get()
  discard tenant
else:
  discard httpResponse
[inline-code-end]
