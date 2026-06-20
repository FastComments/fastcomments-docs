---
## Parametreler

| Name | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |

## Yanıt

Döndürür: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Örnek

[inline-code-attrs-start title = 'deleteNotificationCount Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteNotificationCount(tenantId = "my-tenant-123", id = "notification-789")
if response.isSome:
  let emptyResp = response.get()
  echo "Notification count deleted for tenant: ", "my-tenant-123"
else:
  echo "Failed to delete notification count, status: ", $httpResponse.statusCode
[inline-code-end]

---