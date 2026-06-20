---
## 參數

| 名稱 | 類型 | 是否必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| includeByUserIdAndEmail | boolean | 否 |  |
| includeByIP | boolean | 否 |  |
| includeByEmailDomain | boolean | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`PreBanSummary`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PreBanSummary.ts)

## 範例

[inline-code-attrs-start title = 'getPreBanSummary 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt-9f7b2e3d-45a1';
const includeByUserIdAndEmail: boolean = true;
const includeByIP: boolean = true;
const includeByEmailDomain: boolean = false;
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const summary: PreBanSummary = await getPreBanSummary(
  commentId,
  includeByUserIdAndEmail,
  includeByIP,
  includeByEmailDomain,
  sso
);
[inline-code-end]

---