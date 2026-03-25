## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateNotificationBody | UpdateNotificationBody | Sì |  |
| userId | string | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_8f4b2c';
const id: string = 'notification_61a2e9';
const userId: string = 'moderator_107';
const updateNotificationBody: UpdateNotificationBody = {
  name: 'Flagged Comment Notification',
  enabled: true,
  channels: ['email', 'inbox'],
  templateId: 'tmpl_mod_alerts_01',
  severity: 'high'
};
const result: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
[inline-code-end]

---