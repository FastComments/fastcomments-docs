## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| sso | string | いいえ |  |
| includeUserInfo | PageReactIncludeUserInfo | いいえ |  |

## レスポンス

戻り値: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## 例

[inline-code-attrs-start title = 'getV2PageReacts の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc1234567890';
const urlId: string = '/blog/my-article';

const result: GetV2PageReacts = await getV2PageReacts(tenantId, urlId, 'sso-token-value', 'basic');
[inline-code-end]