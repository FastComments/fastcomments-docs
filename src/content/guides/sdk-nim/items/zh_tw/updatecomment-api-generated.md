## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updatableCommentParams | UpdatableCommentParams | 否 |  |
| options | UpdateCommentOptions | 否 |  |

## 回應

返回: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'updateComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentParams = UpdatableCommentParams(content: "Updated comment content")
let updateOpts = UpdateCommentOptions(force: false)

let (respOpt, httpResp) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  updatableCommentParams = commentParams,
  options = updateOpts
)

if respOpt.isSome:
  let resp = respOpt.get()
[inline-code-end]