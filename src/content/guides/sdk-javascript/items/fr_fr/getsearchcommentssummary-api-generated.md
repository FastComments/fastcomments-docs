## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| value | string | Non |  |
| filters | string | Non |  |
| searchFilters | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationCommentSearchResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSearchCommentsSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const query: string = 'renewable energy incentives';
const filters: string = 'status:approved AND created_at>2025-01-01';
const searchFilters: string | undefined = undefined;
const sso: string | undefined = undefined;
const summary: ModerationCommentSearchResponse = await getSearchCommentsSummary(query, filters, searchFilters, sso);
[inline-code-end]

---