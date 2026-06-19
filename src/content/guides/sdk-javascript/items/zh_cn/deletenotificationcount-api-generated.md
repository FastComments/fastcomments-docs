## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteNotificationCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const idPrefix: string | undefined = 'count';
const notificationId: string = `${idPrefix ? idPrefix + '-' : ''}8b3a9f6c-3e8f-4f6a-a2f3-1a2b3c4d5e6f`;
const tenantId: string = 'acme-media-tenant-42';
const result: APIEmptyResponse = await deleteNotificationCount(tenantId, notificationId);
[inline-code-end]

---