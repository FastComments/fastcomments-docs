## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| userId | string | いいえ |  |
| trustFactor | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetUserTrustFactorResponse.ts)

## 例

[inline-code-attrs-start title = 'setTrustFactor の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = 'user_74219';
const trustFactor: string = 'high';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzc0MjE5In0.signature';

const responseWithoutSso: SetUserTrustFactorResponse = await setTrustFactor(userId, trustFactor);
const responseWithSso: SetUserTrustFactorResponse = await setTrustFactor(userId, trustFactor, ssoToken);
[inline-code-end]

---