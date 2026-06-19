## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| sure | string | Не |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteTenant Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2b9c8a';
const id: string = 'site_9d4f2a1b';
const confirmation: string = 'CONFIRM_DELETE';
const result: APIEmptyResponse = await deleteTenant(tenantId, id, confirmation);
[inline-code-end]

---