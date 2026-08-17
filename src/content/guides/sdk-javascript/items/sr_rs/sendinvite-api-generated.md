## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| fromName | string | Да |  |

## Одговор

Враћа: [`SendInviteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendInviteResponse.ts)

## Пример

[inline-code-attrs-start title = 'sendInvite Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const inviteId: string = "invite-12345";
const fromName: string = "John Doe";

const inviteResult: SendInviteResponse = await sendInvite(tenantId, inviteId, fromName);
[inline-code-end]