## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateNotificationBody | UpdateNotificationBody | 예 |  |
| userId | string | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateNotification 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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