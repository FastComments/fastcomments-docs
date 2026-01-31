## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateTenantBody | UpdateTenantBody | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateTenant Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateTenantBody(
  name = "Acme News",
  domain = "news.acme.com",
  allowedOrigins = @["https://news.acme.com"],
  isActive = true
)
let (response, httpResponse) = client.updateTenant(tenantId = "my-tenant-123", id = "", updateTenantBody = updateBody)
if response.isSome:
  let result = response.get()
  echo "Tenant update response received: ", $result
[inline-code-end]
