List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

Список страниц для арендатора. Используется настольным клиентом FChat для заполнения списка комнат.  
Требует, чтобы `enableFChat` был установлен в `true` в разрешённой пользовательской конфигурации для каждой страницы.  
Страницы, требующие SSO, фильтруются по групповому доступу запрашивающего пользователя.

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Так |  |
| cursor | string | Ні |  |
| limit | number | Ні |  |
| q | string | Ні |  |
| sortBy | PagesSortBy | Ні |  |
| hasComments | boolean | Ні |  |

## Response

Возвращает: [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]