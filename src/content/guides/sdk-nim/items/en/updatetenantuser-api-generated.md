## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantUserBody | UpdateTenantUserBody | No |  |
| updateComments | string = "" | No |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'updateTenantUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateTenantUserBody()
let (optResp, httpResp) = client.updateTenantUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateTenantUserBody = updateBody,
  updateComments = "Changed role to moderator",
)
if optResp.isSome:
  let _ = optResp.get()
[inline-code-end]
