## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| value | string | Nie |  |
| filters | string | Nie |  |
| searchFilters | string | Nie |  |
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetSearchCommentsSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchCommentsSummaryResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getSearchCommentsSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const searchTerm: string = "fastcomments integration";
    const filterString: string = "status:approved";
    const searchFilterString: string = "author:jane";
    const tenantId: string = "123e4567-e89b-12d3-a456-426614174000";
    const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

    const summary: GetSearchCommentsSummaryResponse = await getSearchCommentsSummary(
        searchTerm,
        filterString,
        searchFilterString,
        tenantId,
        ssoToken
    );

    console.log(summary);
}
runExample();
[inline-code-end]