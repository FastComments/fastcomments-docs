## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| value | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`GetSearchPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchPagesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getSearchPages 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const query: string = "network outage";
  const tenantId: string = "tenant-9876";
  const ssoToken: string = "sso-abc123def456";

  const searchResult: GetSearchPagesResponse = await getSearchPages(query, tenantId, ssoToken);
  const searchResultNoSso: GetSearchPagesResponse = await getSearchPages(query, tenantId);
})();
[inline-code-end]