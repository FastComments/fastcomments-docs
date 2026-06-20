## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| page | number | Nie |  |
| count | number | Nie |  |
| textSearch | string | Nie |  |
| byIPFromComment | string | Nie |  |
| filters | string | Nie |  |
| searchFilters | string | Nie |  |
| sorts | string | Nie |  |
| demo | boolean | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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