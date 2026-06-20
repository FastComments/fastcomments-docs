## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| createQuestionConfigBody | CreateQuestionConfigBody | 否 |  |

## 回應

回傳: [`Option[CreateQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_config_response.nim)

## 範例

[inline-code-attrs-start title = 'createQuestionConfig 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createQuestionConfig(
  tenantId = "my-tenant-123",
  createQuestionConfigBody = CreateQuestionConfigBody(
    label = "Article Question",
    required = true,
    minLength = 20,
    maxLength = 1000,
    allowedTags = @["comment","question","feedback"],
    notifyModerators = false
  )
)
if response.isSome:
  let cfg = response.get()
  echo "Created question config id: ", cfg.id
[inline-code-end]

---