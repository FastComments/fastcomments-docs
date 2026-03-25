## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateNotificationBody | UpdateNotificationBody | Ja |  |
| userId | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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