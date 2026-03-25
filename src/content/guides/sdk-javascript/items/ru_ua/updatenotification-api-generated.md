## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateNotificationBody | UpdateNotificationBody | Да |  |
| userId | string | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateNotification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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