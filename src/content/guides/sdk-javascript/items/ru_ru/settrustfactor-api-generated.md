## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| trustFactor | string | No |  |
| sso | string | No |  |

## Ответ

Возвращает: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetUserTrustFactorResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример setTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSetTrustFactor() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";
  const trustFactor: string = "high";
  const sso: string = "sso-token-xyz";

  const fullResponse: SetUserTrustFactorResponse = await setTrustFactor(
    tenantId,
    userId,
    trustFactor,
    sso
  );

  const minimalResponse: SetUserTrustFactorResponse = await setTrustFactor(tenantId);
}
[inline-code-end]