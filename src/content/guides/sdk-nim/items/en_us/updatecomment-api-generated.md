## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updatableCommentParams | UpdatableCommentParams | No |  |
| contextUserId | string | No |  |
| doSpamCheck | bool | No |  |
| isLive | bool | No |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'updateComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654",
  updatableCommentParams = UpdatableCommentParams(
    text = "Updated comment: corrected facts and clarified wording.",
    isApproved = true,
    tags = @["news", "update"]
  ),
  contextUserId = "user-456",
  doSpamCheck = true,
  isLive = true
)

if response.isSome:
  let apiResp = response.get()
  discard apiResp
[inline-code-end]
