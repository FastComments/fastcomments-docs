## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| domain | string | 否 |  |

## 响应

返回: [`Option[DeleteDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config200response.nim)

## 示例

[inline-code-attrs-start title = 'deleteDomainConfig 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let result = response.get()
  echo "Deleted domain config result: ", result
else:
  echo "No response body, HTTP status: ", $httpResponse.status
[inline-code-end]

---