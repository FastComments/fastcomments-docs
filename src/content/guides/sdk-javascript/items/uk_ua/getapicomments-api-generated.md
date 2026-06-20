## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| page | number | Ні |  |
| count | number | Ні |  |
| textSearch | string | Ні |  |
| byIPFromComment | string | Ні |  |
| filters | string | Ні |  |
| searchFilters | string | Ні |  |
| sorts | string | Ні |  |
| demo | boolean | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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