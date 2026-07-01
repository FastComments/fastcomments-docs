## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`DeleteNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteNotificationCountResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteNotificationCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_12345";
  const notificationId: string = "notif_98765";
  const result: DeleteNotificationCountResponse = await deleteNotificationCount(tenantId, notificationId);
  console.log(result);
}
run();
[inline-code-end]

---