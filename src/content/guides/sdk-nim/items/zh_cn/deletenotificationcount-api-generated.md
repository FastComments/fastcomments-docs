## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 响应

返回： [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'deleteNotificationCount 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteNotificationCount(tenantId = "my-tenant-123", id = "notification-789")
if response.isSome:
  let emptyResp = response.get()
  echo "Notification count deleted for tenant: ", "my-tenant-123"
else:
  echo "Failed to delete notification count, status: ", $httpResponse.statusCode
[inline-code-end]

---