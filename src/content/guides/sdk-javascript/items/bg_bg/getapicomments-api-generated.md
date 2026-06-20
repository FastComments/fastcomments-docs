---
## Параметри

| Name | Type | Задължително | Описание |
|------|------|--------------|----------|
| page | number | Не |  |
| count | number | Не |  |
| textSearch | string | Не |  |
| byIPFromComment | string | Не |  |
| filters | string | Не |  |
| searchFilters | string | Не |  |
| sorts | string | Не |  |
| demo | boolean | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const page: number = 2;
const count: number = 25;
const textSearch: string = 'comments failing to load after posting';
const filters: string = 'status:pending,moderation:required';
const sorts: string = 'createdAt:desc';
const demo: boolean = false;
const sso: string = 'sso-usr-7f3b2a';

const response: ModerationAPIGetCommentsResponse = await getApiComments(page, count, textSearch, undefined, filters, undefined, sorts, demo, sso);
[inline-code-end]

---