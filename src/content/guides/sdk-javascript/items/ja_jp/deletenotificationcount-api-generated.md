## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 応答

返却: [`DeleteNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteNotificationCountResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteNotificationCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_12345";
  const notificationId: string = "notif_98765";
  const result: DeleteNotificationCountResponse = await deleteNotificationCount(tenantId, notificationId);
  console.log(result);
}
run();
[inline-code-end]