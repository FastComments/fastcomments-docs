## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateUserBadgeParams | UpdateUserBadgeParams | No |  |

## Response

Returns: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## Example

[inline-code-attrs-start title = 'updateUserBadge Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = UpdateUserBadgeParams()
let (maybeResp, httpResp) = client.updateUserBadge(tenantId = "my-tenant-123", id = "user-456", updateUserBadgeParams = params)
if maybeResp.isSome:
  let success = maybeResp.get()
[inline-code-end]
