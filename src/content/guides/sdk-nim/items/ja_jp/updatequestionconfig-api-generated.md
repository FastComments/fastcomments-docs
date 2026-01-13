## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | いいえ |  |

## レスポンス

返り値: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'updateQuestionConfig の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionConfig(
  tenantId = "my-tenant-123",
  id = "q-config-456",
  updateQuestionConfigBody = UpdateQuestionConfigBody()
)
if response.isSome:
  let updated = response.get()
  discard updated
  echo "Question config updated"
else:
  echo "Update did not return a result"
[inline-code-end]

---