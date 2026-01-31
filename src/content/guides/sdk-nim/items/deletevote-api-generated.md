## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| editKey | string | No |  |

## Response

Returns: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Example

[inline-code-attrs-start title = 'deleteVote Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "cmt-987654321", editKey = "editkey-abc123")
if response.isSome:
  let voteResp = response.get()
  echo "Deleted vote response: ", $voteResp
else:
  echo "Delete failed, HTTP status: ", $httpResponse.status
[inline-code-end]
