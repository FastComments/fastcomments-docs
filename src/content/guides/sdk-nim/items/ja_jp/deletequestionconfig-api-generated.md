## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

戻り値: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'deleteQuestionConfig の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "qcfg-456")
if response.isSome:
  let respVal = response.get()
  echo "Delete succeeded for tenant my-tenant-123"
else:
  echo "Delete returned no data (status: ", $httpResponse.status, ")"
[inline-code-end]

---