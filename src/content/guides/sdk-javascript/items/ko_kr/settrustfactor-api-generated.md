## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| userId | string | 아니오 |  |
| trustFactor | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetUserTrustFactorResponse.ts)

## 예제

[inline-code-attrs-start title = 'setTrustFactor 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = 'user_74219';
const trustFactor: string = 'high';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzc0MjE5In0.signature';

const responseWithoutSso: SetUserTrustFactorResponse = await setTrustFactor(userId, trustFactor);
const responseWithSso: SetUserTrustFactorResponse = await setTrustFactor(userId, trustFactor, ssoToken);
[inline-code-end]

---