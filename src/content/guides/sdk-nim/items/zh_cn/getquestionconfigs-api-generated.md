## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 响应

返回：[`Option[GetQuestionConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs_response.nim)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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