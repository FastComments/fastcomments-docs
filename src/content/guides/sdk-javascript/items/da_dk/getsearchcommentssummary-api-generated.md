## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| value | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sso | string | No |  |

## Svar

Returns: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationCommentSearchResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getSearchCommentsSummary Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runSearch() {
  const tenantId: string = "tenant_12345";
  const value: string = "spam";
  const filters: string = "status:pending";
  const searchFilters: string = "author:john";
  const sso: string = "sso_token_abc";

  const result: ModerationCommentSearchResponse = await getSearchCommentsSummary(
    tenantId,
    value,
    filters,
    searchFilters,
    sso
  );

  console.log(result);
}

runSearch();
[inline-code-end]