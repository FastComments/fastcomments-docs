## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| userId | string | 아니오 |  |
| trustFactor | string | 아니오 |  |
| tenantId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환값: [`SetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetTrustFactorResponse.ts)

## 예제

[inline-code-attrs-start title = 'setTrustFactor 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const userId: string = "user_8421";
  const trustFactor: string = "high";
  const tenantId: string = "tenant_33";
  const ssoToken: string = "sso_7d9f";

  const fullResult: SetTrustFactorResponse = await setTrustFactor(userId, trustFactor, tenantId, ssoToken);
  const minimalResult: SetTrustFactorResponse = await setTrustFactor(userId, trustFactor);
})();
[inline-code-end]