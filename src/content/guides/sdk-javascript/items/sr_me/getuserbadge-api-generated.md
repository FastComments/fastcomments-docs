## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Пример

[inline-code-attrs-start title = 'getUserBadge Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const id: string = "badge_8c7d2f";
const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
[inline-code-end]