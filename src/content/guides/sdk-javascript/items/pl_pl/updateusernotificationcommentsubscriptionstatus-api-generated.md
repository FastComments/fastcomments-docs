Włącz lub wyłącz powiadomienia dla konkretnego komentarza.

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| notificationId | string | Tak |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Tak |  |
| commentId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserNotificationCommentSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_92a1c4';
const notificationId: string = 'notif_3f7b2e9a-4d1b-11ec-8d3d-0242ac130003';
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum =
  UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OPTED_IN;
const commentId: string = 'cmt_8f4b2e7a';
const sso: string | undefined = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload';
const response: UpdateUserNotificationCommentSubscriptionStatusResponse =
  await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]