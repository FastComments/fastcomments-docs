---
## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | はい |  |
| includeByUserIdAndEmail | boolean | いいえ |  |
| includeByIP | boolean | いいえ |  |
| includeByEmailDomain | boolean | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkPreBanSummary.ts)

## 例

[inline-code-attrs-start title = 'postBulkPreBanSummary の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const bulkParams: BulkPreBanParams = {
  accounts: [
    { userId: 'u_8729', email: 'bot123@malicious.com', ip: '192.0.2.55' },
    { userId: 'u_9901', email: 'spam.sender@cheapmeds.co', ip: '198.51.100.12' }
  ],
  reason: 'Automated pre-ban candidate import'
};
const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = true;
const includeByEmailDomain: boolean = false;
const sso: string = 'sso_58fd3b2c-token';
const result: BulkPreBanSummary = await postBulkPreBanSummary(bulkParams, includeByUserIdAndEmail, includeByIP, includeByEmailDomain, sso);
[inline-code-end]

---