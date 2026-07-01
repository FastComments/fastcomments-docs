## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| userId | string | Не |  |
| trustFactor | string | Не |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`SetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetTrustFactorResponse.ts)

## Пример

[inline-code-attrs-start title = 'setTrustFactor Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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