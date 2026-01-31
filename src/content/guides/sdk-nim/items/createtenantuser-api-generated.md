## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | No |  |

## Response

Returns: [`Option[CreateTenantUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_user200response.nim)

## Example

[inline-code-attrs-start title = 'createTenantUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let createUserBody = CreateTenantUserBody(email = "alice.johnson@acme.com", displayName = "Alice Johnson", roles = @["moderator"], enabled = true)
let (response, httpResponse) = client.createTenantUser(tenantId = tenantId, createTenantUserBody = createUserBody)
if response.isSome:
  let user = response.get()
  discard user
[inline-code-end]
