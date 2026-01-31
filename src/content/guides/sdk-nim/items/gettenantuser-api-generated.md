## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[GetTenantUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_user200response.nim)

## Example

[inline-code-attrs-start title = 'getTenantUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUser(tenantId = "my-tenant-123", id = "user-7f9c-42")
if response.isSome:
  let user = response.get()
  echo "Tenant user retrieved:"
  echo "  id: ", user.id
  echo "  email: ", user.email
else:
  echo "User not found, status: ", $httpResponse.status
[inline-code-end]
