## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Ответ

Возвращает: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // страница
    25,                    // количество
    "feedback",           // поиск по тексту
    "192.168.1.100",      // по IP из комментария
    "approved",           // фильтры
    "hasReplies",         // фильтры поиска
    "dateDesc",           // сортировка
    false,                // демо
    "tenant-abc123",      // идентификатор арендатора
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]