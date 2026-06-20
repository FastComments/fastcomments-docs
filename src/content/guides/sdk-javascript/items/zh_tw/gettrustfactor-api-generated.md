## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| userId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserTrustFactorResponse.ts)

## 範例

[inline-code-attrs-start title = 'getTrustFactor 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = '550e8400-e29b-41d4-a716-446655440000';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.VXNlckRhdGE.signature';
const trustFactor: GetUserTrustFactorResponse = await getTrustFactor(userId, sso);
const trustFactorAnonymous: GetUserTrustFactorResponse = await getTrustFactor();
[inline-code-end]

---