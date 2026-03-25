## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Пример

[inline-code-attrs-start title = 'getModerator Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-123';
const id: string = 'mod-987654321';
const moderatorResponse: GetModerator200Response = await getModerator(tenantId, id);
[inline-code-end]

---