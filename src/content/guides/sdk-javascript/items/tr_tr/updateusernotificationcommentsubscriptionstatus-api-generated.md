Belirli bir yorum için bildirimleri etkinleştirin veya devre dışı bırakın.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| notificationId | string | Evet |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Evet |  |
| commentId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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