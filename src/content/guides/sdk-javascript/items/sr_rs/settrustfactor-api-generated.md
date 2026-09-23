## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| trustFactor | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetUserTrustFactorResponse.ts)

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