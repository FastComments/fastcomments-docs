## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| deleteComments | string | いいえ |  |
| commentDeleteMode | string | いいえ |  |

## レスポンス

戻り値: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'deleteTenantUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(tenantId = "my-tenant-123", id = "user-456", deleteComments = "", commentDeleteMode = "")
if response.isSome:
  let flagResp = response.get()
  echo flagResp
[inline-code-end]

---