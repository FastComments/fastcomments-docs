## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateNotificationBody | UpdateNotificationBody | 是 |  |
| userId | string | 否 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateNotification 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_86a7b3";
const id: string = "notif_20260112_01";
const userId: string = "moderator_42";
const updateNotificationBody: UpdateNotificationBody = {
  name: "Flagged comment alert",
  enabled: true,
  channels: ["email"],
  recipients: ["mod-team@news-site.com"],
  threshold: 1
};

(async () => {
  const result: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
  console.log(result);
})();
[inline-code-end]

---