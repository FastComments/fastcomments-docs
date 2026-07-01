## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| page | number | Нет |  |
| count | number | Нет |  |
| textSearch | string | Нет |  |
| byIPFromComment | string | Нет |  |
| filters | string | Нет |  |
| searchFilters | string | Нет |  |
| sorts | string | Нет |  |
| demo | boolean | Нет |  |
| tenantId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // страница
    25,                    // количество
    "feedback",           // поискТекста
    "192.168.1.100",      // поIPИзКомментария
    "approved",           // фильтры
    "hasReplies",         // фильтрыПоиска
    "dateDesc",           // сортировки
    false,                // демо
    "tenant-abc123",      // tenantId
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]