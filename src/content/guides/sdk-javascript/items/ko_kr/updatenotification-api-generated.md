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
const tenantId: string = 'tenant_9f3b2c';
const id: string = 'notification_4a1d2e';
const updateNotificationBody: UpdateNotificationBody = {
  enabled: true,
  channels: ['email', 'push'],
  frequency: 'immediate',
  templateId: 'tmpl_77aa'
} as UpdateNotificationBody;
const userId: string = 'user_2468';
const result: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
[inline-code-end]