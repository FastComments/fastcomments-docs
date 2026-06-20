## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| banEmail | bool | いいえ |  |
| banEmailDomain | bool | いいえ |  |
| banIP | bool | いいえ |  |
| deleteAllUsersComments | bool | いいえ |  |
| bannedUntil | string | いいえ |  |
| isShadowBan | bool | いいえ |  |
| updateId | string | いいえ |  |
| banReason | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## 例

[inline-code-attrs-start title = 'postBanUserFromComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postBanUserFromComment(
  commentId = "cmt-8f3a1b",
  banEmail = false,
  banEmailDomain = false,
  banIP = false,
  deleteAllUsersComments = false,
  bannedUntil = "",
  isShadowBan = false,
  updateId = "",
  banReason = "",
  sso = ""
)
if response.isSome:
  let banResult = response.get()
  discard banResult
else:
  echo "No ban result returned"
[inline-code-end]

---