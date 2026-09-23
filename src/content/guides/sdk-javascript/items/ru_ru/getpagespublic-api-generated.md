---
Список страниц для арендатора. Используется клиентом FChat для настольных компьютеров для заполнения списка комнат.  
Требует, чтобы `enableFChat` был установлен в true в разрешённой пользовательской конфигурации для каждой страницы.  
Страницы, требующие SSO, фильтруются в соответствии с доступом группы запрашивающего пользователя.

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Нет |  |
| limit | number | Нет |  |
| q | string | Нет |  |
| sortBy | PagesSortBy | Нет |  |
| hasComments | boolean | Нет |  |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]

---