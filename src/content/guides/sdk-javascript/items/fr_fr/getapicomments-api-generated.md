## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| page | number | Non |  |
| count | number | Non |  |
| textSearch | string | Non |  |
| byIPFromComment | string | Non |  |
| filters | string | Non |  |
| searchFilters | string | Non |  |
| sorts | string | Non |  |
| demo | boolean | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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