## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| userId | string | No |  |
| trustFactor | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Resposta

Retorna: [`SetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetTrustFactorResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo setTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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