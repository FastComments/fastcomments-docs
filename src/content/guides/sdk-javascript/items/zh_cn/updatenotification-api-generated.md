## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateNotificationBody | UpdateNotificationBody | 是 |  |
| userId | string | 否 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 示例

[inline-code-attrs-start title = 'updateNotification 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = "tenant-abc123";
  const notificationId: string = "notif-9876";
  const updateBody: UpdateNotificationBody = {
    title: "System Maintenance",
    message: "Scheduled maintenance at 02:00 UTC.",
    enabled: true
  };
  const userId: string = "user-42";

  const resultWithoutUser: APIEmptyResponse = await updateNotification(tenantId, notificationId, updateBody);
  const resultWithUser: APIEmptyResponse = await updateNotification(tenantId, notificationId, updateBody, userId);
}
[inline-code-end]