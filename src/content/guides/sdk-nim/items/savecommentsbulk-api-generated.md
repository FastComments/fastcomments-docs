## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | seq[CreateCommentParams] | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| sendEmails | bool | No |  |
| populateNotifications | bool): (Option[seq[SaveComment_200_response]] | No |  |
| id | string | No |  |
| fromName | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'saveCommentsBulk Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.saveCommentsBulk(
  tenantId = "my-tenant-123",
  createCommentParams = @[],
  isLive = true,
  doSpamCheck = true,
  sendEmails = false,
  populateNotifications = true,
  id = "batch-2026-01-09",
  fromName = "News Importer"
)
if response.isSome:
  let flagged = response.get()
  discard flagged
[inline-code-end]
