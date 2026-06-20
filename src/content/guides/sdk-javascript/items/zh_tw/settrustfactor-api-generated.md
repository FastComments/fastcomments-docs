## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| userId | string | 否 |  |
| trustFactor | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetUserTrustFactorResponse.ts)

## 範例

[inline-code-attrs-start title = 'setTrustFactor 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = 'user_74219';
const trustFactor: string = 'high';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzc0MjE5In0.signature';

const responseWithoutSso: SetUserTrustFactorResponse = await setTrustFactor(userId, trustFactor);
const responseWithSso: SetUserTrustFactorResponse = await setTrustFactor(userId, trustFactor, ssoToken);
[inline-code-end]

---