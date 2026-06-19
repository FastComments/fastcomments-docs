---
Список страниц для арендатора. Используется настольным клиентом FChat для заполнения списка комнат.
Требует, чтобы в итоговой пользовательской конфигурации для каждой страницы `enableFChat` было true.
Страницы, требующие SSO, отфильтровываются в соответствии с групповым доступом запрашивающего пользователя.

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Нет |  |
| limit | number | Нет |  |
| q | string | Нет |  |
| sortBy | PagesSortBy | Нет |  |
| hasComments | boolean | Нет |  |

## Response

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---