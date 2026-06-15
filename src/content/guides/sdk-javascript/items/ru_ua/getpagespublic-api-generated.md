Перечисляет страницы для тенанта. Используется настольным клиентом FChat для заполнения списка комнат.
Требуется, чтобы `enableFChat` было равно true в итоговой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются по доступу групп пользователя, делающего запрос.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Нет |  |
| limit | number | Нет |  |
| q | string | Нет |  |
| sortBy | PagesSortBy | Нет |  |
| hasComments | boolean | Нет |  |

## Response

Возвращает: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]