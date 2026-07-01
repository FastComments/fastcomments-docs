## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| value | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 响应

返回: [`GetSearchPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchPagesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getSearchPages 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const query: string = "network outage";
  const tenantId: string = "tenant-9876";
  const ssoToken: string = "sso-abc123def456";

  const searchResult: GetSearchPagesResponse = await getSearchPages(query, tenantId, ssoToken);
  const searchResultNoSso: GetSearchPagesResponse = await getSearchPages(query, tenantId);
})();
[inline-code-end]