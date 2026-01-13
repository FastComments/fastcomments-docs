## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## 示例

[inline-code-attrs-start title = 'getCachedNotificationCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const baseNotificationId: string = 'notif-000123';
const idSuffix: string | undefined = undefined; // 可选参数示例
const notificationId: string = idSuffix ? `${baseNotificationId}-${idSuffix}` : baseNotificationId;
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, notificationId);
console.log(result);
[inline-code-end]

---