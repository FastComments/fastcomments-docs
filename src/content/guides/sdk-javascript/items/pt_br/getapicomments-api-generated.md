## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| page | number | Não |  |
| count | number | Não |  |
| textSearch | string | Não |  |
| byIPFromComment | string | Não |  |
| filters | string | Não |  |
| searchFilters | string | Não |  |
| sorts | string | Não |  |
| demo | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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