## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 响应

返回: [`Option[GetQuestionConfigs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs200response.nim)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let configs = response.get()
  discard configs
[inline-code-end]

---