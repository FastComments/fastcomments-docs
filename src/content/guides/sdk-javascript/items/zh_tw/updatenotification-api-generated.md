## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateNotificationBody | UpdateNotificationBody | Yes |  |
| userId | string | No |  |

## 回應

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateNotification 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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