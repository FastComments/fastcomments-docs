## Параметри

| Назив | Тип | Захтевано | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| sure | string | Не |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42c9f1';
const id: string = 'flag_9a7b3c';
const sure: string = 'confirm-delete';
const result: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
[inline-code-end]

---