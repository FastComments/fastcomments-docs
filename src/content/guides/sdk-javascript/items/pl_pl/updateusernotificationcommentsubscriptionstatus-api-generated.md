Włącz lub wyłącz powiadomienia dla konkretnego komentarza.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | Yes |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Odpowiedź

Zwraca: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserNotificationCommentSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const notificationId: string = 'notif-2026-03-25-01';
const commentId: string = 'cmt-8f3a2b';
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum = UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OptIn;
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso-payload.signature';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]

---