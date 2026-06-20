## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| spam | bool | いいえ |  |
| permNotSpam | bool | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'postSetCommentSpamStatus の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentSpamStatus(
  commentId = "cmt-20250619-842",
  spam = false,
  permNotSpam = false,
  sso = ""
)
if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---