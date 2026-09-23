## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| userId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserTrustFactorResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_98765";
  const sso: string = "sso_token_abcde";

  const trustFactor: GetUserTrustFactorResponse = await getTrustFactor(tenantId);
  const trustFactorWithUser: GetUserTrustFactorResponse = await getTrustFactor(tenantId, userId);
  const trustFactorFull: GetUserTrustFactorResponse = await getTrustFactor(tenantId, userId, sso);
}
[inline-code-end]