## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| options | PostSetCommentReviewStatusOptions | 否 |  |

## 响应

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'postSetCommentReviewStatus 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postSetCommentReviewStatus(
  tenantId = "my-tenant-123",
  commentId = "cmt-7890",
  options = PostSetCommentReviewStatusOptions()
)

if apiResp.isSome:
  let _ = apiResp.get()
  discard
else:
  discard
[inline-code-end]