## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteNotificationCount Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId: string = "tenant-42f9b8c1";
let notificationId: string = "notif-7e3a9d4f";

const result: APIEmptyResponse = await deleteNotificationCount(tenantId, notificationId);
[inline-code-end]

---