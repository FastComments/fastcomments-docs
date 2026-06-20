## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | No |  |
| title | string | No |  |

## 响应

返回: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## 示例

[inline-code-attrs-start title = 'createV2PageReact 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/2026/06/fastcomments-release",
  id = "",
  title = ""
)
if response.isSome:
  let react = response.get()
  echo "Created page react: ", $react
else:
  echo "No react returned, HTTP status: ", $httpResponse.statusCode
[inline-code-end]

---