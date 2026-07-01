## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| sendEmail | string = "" | 否 |  |

## 响应

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'deleteModerator 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteModerator(
  tenantId = "my-tenant-123",
  id = "mod-789",
  sendEmail = "admin@mydomain.com",
)

if apiResp.isSome:
  let empty = apiResp.get()
  echo "Moderator removed"
[inline-code-end]

---