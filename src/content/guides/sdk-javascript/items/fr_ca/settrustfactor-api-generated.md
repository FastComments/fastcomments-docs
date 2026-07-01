## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| userId | string | Non |  |
| trustFactor | string | Non |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`SetTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetTrustFactorResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple setTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---