## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostSetCommentSpamStatusOptions | No |  |

## レスポンス

返却: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'postSetCommentSpamStatus の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let defaultOpts = PostSetCommentSpamStatusOptions()
let (maybeResp, httpResp) = client.postSetCommentSpamStatus(tenantId = "my-tenant-123", commentId = "cmt-456789", options = defaultOpts)
if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]