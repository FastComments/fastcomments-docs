## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| fromName | string | Так |  |

## Відповідь

Повертає: [`SendInviteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendInviteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'sendInvite Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const inviteId: string = "invite-12345";
const fromName: string = "John Doe";

const inviteResult: SendInviteResponse = await sendInvite(tenantId, inviteId, fromName);
[inline-code-end]