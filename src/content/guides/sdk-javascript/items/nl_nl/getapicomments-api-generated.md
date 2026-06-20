## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| page | number | Nee |  |
| count | number | Nee |  |
| textSearch | string | Nee |  |
| byIPFromComment | string | Nee |  |
| filters | string | Nee |  |
| searchFilters | string | Nee |  |
| sorts | string | Nee |  |
| demo | boolean | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getApiComments Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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