## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| sure | string | Не |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'deleteTenant Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_742b9c';
const flagId: string = 'flag_1a2b3c';
const resultWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId);
const sureConfirmation: string = 'confirmed';
const resultWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, flagId, sureConfirmation);
[inline-code-end]

---