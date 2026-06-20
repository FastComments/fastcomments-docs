## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 回應

回傳: [`Option[GetQuestionConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs_response.nim)

## 範例

[inline-code-attrs-start title = 'getQuestionConfigs 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let configs = response.get()
  echo "Received question configs for tenant my-tenant-123"
  echo configs
else:
  echo "No question configs returned"
[inline-code-end]

---