## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| value | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Одговор

Враћа: [`GetSearchCommentsSummaryResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchCommentsSummaryResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getSearchCommentsSummary'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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