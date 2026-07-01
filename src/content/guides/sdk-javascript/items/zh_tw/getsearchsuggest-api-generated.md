## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| textSearch | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`GetSearchSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchSuggestResponse.ts)

## 範例

[inline-code-attrs-start title = 'getSearchSuggest 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example(): Promise<void> {
    const term: string = "fastcomments api";
    const tenant: string = "tenant_001";
    const sso: string = "sso_9a8b7c";

    const fullResult: GetSearchSuggestResponse = await getSearchSuggest(term, tenant, sso);
    const partialResult: GetSearchSuggestResponse = await getSearchSuggest(term);
}

example();
[inline-code-end]