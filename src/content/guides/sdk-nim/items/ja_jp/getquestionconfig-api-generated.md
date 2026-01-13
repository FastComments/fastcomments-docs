## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## レスポンス

戻り値: [`Option[GetQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config200response.nim)

## 例

[inline-code-attrs-start title = 'getQuestionConfig の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qcfg-98765")
if response.isSome:
  let config = response.get()
  echo "Received question config for tenant:", " my-tenant-123"
[inline-code-end]

---