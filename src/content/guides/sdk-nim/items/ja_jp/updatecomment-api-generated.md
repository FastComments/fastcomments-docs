## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updatableCommentParams | UpdatableCommentParams | いいえ |  |
| contextUserId | string | いいえ |  |
| doSpamCheck | bool | いいえ |  |
| isLive | bool | いいえ |  |

## レスポンス

戻り値: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'updateComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updatableCommentParams = UpdatableCommentParams(content: "Fixed a typo in the second paragraph", tags: @["article-edit", "typo"], isApproved: true)
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  updatableCommentParams = updatableCommentParams,
  contextUserId = "user-789",
  doSpamCheck = true,
  isLive = true
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---