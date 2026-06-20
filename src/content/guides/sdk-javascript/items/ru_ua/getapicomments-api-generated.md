## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| page | number | Нет |  |
| count | number | Нет |  |
| textSearch | string | Нет |  |
| byIPFromComment | string | Нет |  |
| filters | string | Нет |  |
| searchFilters | string | Нет |  |
| sorts | string | Нет |  |
| demo | boolean | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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