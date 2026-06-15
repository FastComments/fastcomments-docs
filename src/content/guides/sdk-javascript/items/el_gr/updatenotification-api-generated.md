## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| updateNotificationBody | UpdateNotificationBody | Ναι |  |
| userId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---