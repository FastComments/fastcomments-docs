## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSiteSearchResponse.ts)

## 範例

[inline-code-attrs-start title = 'getSearchSites 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = 'fastcomments.com';
const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const responseWithSSO: ModerationSiteSearchResponse = await getSearchSites(searchValue, ssoToken);
const responseWithoutSSO: ModerationSiteSearchResponse = await getSearchSites('news.fastcompany.com');
[inline-code-end]

---