## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| value | string | Не |  |
| filters | string | Не |  |
| searchFilters | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationCommentSearchResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getSearchCommentsSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const query: string = 'renewable energy incentives';
const filters: string = 'status:approved AND created_at>2025-01-01';
const searchFilters: string | undefined = undefined;
const sso: string | undefined = undefined;
const summary: ModerationCommentSearchResponse = await getSearchCommentsSummary(query, filters, searchFilters, sso);
[inline-code-end]

---