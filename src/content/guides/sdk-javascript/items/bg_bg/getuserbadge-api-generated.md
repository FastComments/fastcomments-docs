## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Отговор

Връща: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const id: string = "badge_8c7d2f";
const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
[inline-code-end]

---