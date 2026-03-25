## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Отговор

Връща: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-123';
const id: string = 'mod-987654321';
const moderatorResponse: GetModerator200Response = await getModerator(tenantId, id);
[inline-code-end]

---