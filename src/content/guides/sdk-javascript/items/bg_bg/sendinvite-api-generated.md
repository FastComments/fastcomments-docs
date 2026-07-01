## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| fromName | string | Да |  |

## Response

Връща: [`SendInviteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SendInviteResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример за sendInvite'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const inviteId: string = "invite-12345";
const fromName: string = "John Doe";

const inviteResult: SendInviteResponse = await sendInvite(tenantId, inviteId, fromName);
[inline-code-end]