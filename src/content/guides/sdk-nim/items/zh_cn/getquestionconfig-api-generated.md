## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 响应

返回：[`Option[GetQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config200response.nim)

## 示例

[inline-code-attrs-start title = 'getQuestionConfig 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qcfg-98765")
if response.isSome:
  let config = response.get()
  echo "Received question config for tenant:", " my-tenant-123"
[inline-code-end]

---