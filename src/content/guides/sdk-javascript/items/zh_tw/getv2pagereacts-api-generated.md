## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| sso | string | 否 |  |
| includeUserInfo | PageReactIncludeUserInfo | 否 |  |

## 回應

回傳：[`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## 範例

[inline-code-attrs-start title = 'getV2PageReacts 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc1234567890';
const urlId: string = '/blog/my-article';

const result: GetV2PageReacts = await getV2PageReacts(tenantId, urlId, 'sso-token-value', 'basic');
[inline-code-end]