## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| fromName | string | Да |  |

## Отговор

Връща: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за sendInvite'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-42';
const id: string = 'comment_8f3b21a7';
const fromName: string = 'Elena Morales';
const replyToEmail: string | undefined = undefined;

const result: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName, replyToEmail);
[inline-code-end]

---