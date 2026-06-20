## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| approved | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Example

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentApprovalStatus(commentId = "cmt-7890", approved = false, sso = "")
if response.isSome:
  let setResp = response.get()
  discard setResp
[inline-code-end]
